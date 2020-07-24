use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Error, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

const BLK_SIZE: usize = 4 * 1024;
const R_THREAD_NUM: usize = 4;

struct Package {
    content: Vec<u8>,
    offset: usize,
    size: usize,
}

impl Package {
    fn new(content: &[u8], offset: usize, size: usize) -> Self {
        let mut p = Package {
            content: vec![0; size],
            offset,
            size,
        };
        p.content.copy_from_slice(content);
        p
    }
}

pub fn read_write_file() -> Result<(), Error> {
    let source = "/tmp/source";
    let dest = "/tmp/destination";
    let mut content = [0; BLK_SIZE];
    let file1 = File::open(source)?;
    let file2 = File::create(dest)?;
    let mut rbuf = BufReader::new(file1);
    let mut wbuf = BufWriter::new(file2);
    let mut size = rbuf.read(&mut content)?;
    wbuf.write(&content[..size])?;
    // buffer.read_exact(&mut content)?; // will panic
    // println!("{:?}", &content[..BLK_SIZE]);
    size = rbuf.read(&mut content)?;
    wbuf.write(&content[..size])?;
    wbuf.flush()?;
    Ok(())
}

fn write_offset() -> Result<(), Error> {
    let filepath = Path::new("/tmp/offset");
    if filepath.exists() {
        fs::remove_file(&filepath)?;
    }
    let file = File::create(&filepath)?;
    let mut writer = BufWriter::new(file);
    let content = "Hello World!";
    writer.seek(SeekFrom::Start(1024))?;
    writer.write(content.as_bytes())?;
    writer.flush()?;

    let file = File::open(&filepath)?;
    let mut reader = BufReader::new(file);
    let mut buffer = vec![0; 1024];
    let mut size = reader.read(&mut buffer).unwrap();
    println!("{:?}", &buffer[..size]);
    size = reader.read(&mut buffer).unwrap();
    println!("{:?}", &buffer[..size]);
    Ok(())
}

fn concurrent_write_offset() -> Result<(), Error> {
    let target = Path::new("/tmp/bigfile");
    if target.exists() {
        fs::remove_file(&target)?;
    }
    let file = File::create(target)?;
    let writer = BufWriter::new(file);
    // let mut writer = Arc::new(writer);
    let writer = Arc::new(Mutex::new(writer));
    // let source = Path::new("/home/howard/Downloads/elasticsearch-7.6.0-amd64.deb");
    let source = Path::new("/home/howard/Downloads/[Mabors-Sub][Re Zero kara Hajimeru Isekai Seikatsu Hyouketsu no Kizuna][OVA][1080P][GB][BDrip][AVC AAC YUV420P8].mp4");
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut thread_pool = Vec::new();
    for i in 0..R_THREAD_NUM {
        let receiver = receiver.clone();
        let writer = writer.clone();
        thread_pool.push(thread::spawn(move || {
            println!("Thread {} started", i);
            loop {
                let mut writer = writer.lock().unwrap();
                // let mut writer = writer.get_mut();
                let receiver = receiver.lock().unwrap();
                let package: Package = receiver.recv().unwrap();
                writer.seek(SeekFrom::Start(package.offset as u64)).unwrap();
                writer.write(&package.content).unwrap();
                println!("Thread {} write {}", i, package.offset);
            }
        }));
    }

    let mut content = [0; BLK_SIZE];
    let mut size = BLK_SIZE;
    let file1 = File::open(source)?;
    let mut rbuf = BufReader::new(file1);
    let mut offset = 0;
    while size == BLK_SIZE {
        size = rbuf.read(&mut content)?;
        // let mut stream = vec![0; size];
        // stream.copy_from_slice(&content[..size]);
        let package = Package::new(&content[..size], offset, size);
        sender.send(package).unwrap();
        offset += BLK_SIZE;
    }

    drop(sender);
    for thread in thread_pool {
        thread.join().unwrap();
    }

    Ok(())
}

// fn thread_read_write_file<P: AsRef<Path>>(source: P, dest: P) -> Result<(), Error> {
//     let (tx, rx) = mpsc::channel();
//     let dest = dest.as_ref().to_path_buf();
//     let handle = thread::spawn(move || {
//         let file2 = File::create(dest).unwrap();
//         let mut wbuf = BufWriter::new(file2);
//         let mut content: Vec<u8> = rx.recv().unwrap();
//         while content.len() == BLK_SIZE {
//             wbuf.write(&content).unwrap();
//             content = rx.recv().unwrap();
//         }
//         wbuf.write(&content).unwrap();
//         wbuf.flush().unwrap();
//     });

//     let mut content = [0; BLK_SIZE];
//     let mut size = BLK_SIZE;
//     let file1 = File::open(source)?;
//     let mut rbuf = BufReader::new(file1);
//     while size == BLK_SIZE {
//         size = rbuf.read(&mut content)?;
//         let mut stream = vec![0; size];
//         stream.copy_from_slice(&content[..size]);
//         tx.send(stream).unwrap();
//     }

//     handle.join().unwrap();
//     Ok(())
// }

// #[test]
// fn test_thread_read_write_file() {
//     let mut source = env::current_dir().unwrap();
//     let mut dest = source.clone();
//     source.push("source");
//     dest.push("destination");
//     if dest.exists() {
//         let file = File::create(&dest).unwrap();
//         file.set_len(0).unwrap();
//         file.sync_data().unwrap();
//     }

//     assert_eq!(thread_read_write_file(&source, &dest).unwrap(), ());
//     let mut source_content = String::new();
//     let mut dest_content = String::new();
//     File::open(&source)
//         .unwrap()
//         .read_to_string(&mut source_content)
//         .unwrap();
//     File::open(&dest)
//         .unwrap()
//         .read_to_string(&mut dest_content)
//         .unwrap();
//     assert_eq!(source_content, dest_content);
// }

#[cfg(test)]
mod file {
    use super::*;

    #[test]
    fn test_read_write_file() {
        let source = Path::new("/tmp/source");
        let dest = Path::new("/tmp/destination");
        let content = "Hello World!";
        if !source.exists() {
            File::create(source)
                .unwrap()
                .write(content.as_bytes())
                .unwrap();
        }
        if dest.exists() {
            fs::remove_file(&dest).unwrap();
        }

        assert_eq!(read_write_file().unwrap(), ());
        let mut dest_content = String::new();
        File::open(&dest)
            .unwrap()
            .read_to_string(&mut dest_content)
            .unwrap();
        assert_eq!(content, dest_content.as_str());
    }
}
