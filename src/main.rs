use std::collections::{HashSet, VecDeque};
use std::{fs, thread};
use std::fs::{copy, create_dir_all, read_dir, DirEntry, ReadDir};
use std::io::stdin;
use std::path::{ PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::JoinHandle;
use std::time::Duration;
use serde::{Deserialize, Serialize};



fn main() {
    /*
     read configuration here and moveing the config instance
    */
    let config: Config = read_configuration();

    let exclude:&HashSet<String> = &config.exclude;
    let thread_allowed =config.multi_threading;

    /*
    [This block handles percentage calculation.]

    Normally, to show accurate progress like Windows does,
    you’d traverse all files, calculate the total size,
    and then update the progress bar or “glow” effect
    as data is copied.

    However, we’re skipping the expensive full traversal.

    Instead, we estimate progress based only on the top-level
    folders in the source directory:
    - Each top-level folder is considered one “unit” of work.
    - When a top-level folder is fully processed (including
      everything inside it), we increment the percentage
      accordingly.

    This is faster, but less precise than calculating by file size.
*/
    let read_dir_instance=read_dir(&config.source_path).unwrap();

    let root_files_count:u32 = count(read_dir_instance);

    //ealry exit so we avoid futher inittializations
    if(root_files_count == 0){
        println!("nothing to process");
        press_any_key_to_exit();
        std::process::exit(1);
    }

    let mut how_much_completed:u32= 0;
    let mut stack_length:u32= 1;


    let file_operation: fn(from: PathBuf, to: PathBuf)  = match config.process.as_str() {
        "copy" => copy_files,
        "move" => move_file,
        _ => {
            eprintln!("Unknown process type");
            press_any_key_to_exit();
            std::process::exit(1);
        }
    };

    // let operation_source:fn(dest:&PathBuf,subscribe:&Subscribe ,file_oper:&fn(from: PathBuf, to: PathBuf),item:DirEntry) = if config.multi_threading {
    //
    // }else {
    //
    // }

    /*
        [Core traversal handler.]

        We avoid the typical recursive traversal approach because
        deeply nested folders could cause a stack overflow.

        Instead, we use a queue (`VecDeque`) to manage directories:
        - Each time we find a subfolder, we add its `ReadDir` iterator
          to the queue.
        - Once the current folder is fully processed, we remove it
          from the queue and continue with the next one.
        - This approach processes directories iteratively rather than
          recursively, which is safer and more memory-efficient.

        The key idea:
        - If we find a folder during the loop, we enqueue it and restart
          the loop, effectively working from the inner folders back
          toward the root.

        Diagram of traversal order:

            [RootFolder] → process files
                 │
                 ├──> enqueue [SubFolder1]
                 │
                 ├──> enqueue [SubFolder2]
                 │
                 ▼
            Process [SubFolder1], enqueue its subfolders...
            Process [SubFolder2], enqueue its subfolders...
            Continue until queue is empty.

        `destination_root` is used to efficiently handle destination paths.
        This way, when copying files, we don’t need to repeatedly convert
        file locations to strings — we can work directly with `PathBuf`.

        `destination_root` stores the base destination path (e.g., E:/name),
         so we can join subpaths directly instead of replacing the source
         root (e.g., D:/name) with the destination root for every file which is not efficient.
    */
    let mut queue:VecDeque<ReadDir> = VecDeque::with_capacity(1000);
    let mut destination_root:PathBuf = PathBuf::from(&config.destination_path);



    let root_folder=read_dir(&config.source_path).unwrap();

    queue.push_back(root_folder);



    /*
     [Ensure the destination directory exists.]

    If the destination path is not already a directory,
    create it (including any missing parent directories).
    */
    if destination_root.is_dir() {
        create_dir_all(&destination_root);
    }


    // std::process::Command::new("cmd")
    //     .args(&["/C", "cls"])
    //     .status()
    //     .unwrap();


    //let fn_deque: Arc<Mutex<VecDeque<Box<dyn FnOnce() + Send>>>> = Arc::new(Mutex::new(VecDeque::new()));
    //already stack size is alocated so ,light heap initalization so not moved inside thread for now

    let subscribe:Subscribe = Subscribe::new();

    let mut handler: Option<Vec<JoinHandle<_>>> = None;

    if(thread_allowed){
        handler = create_thread(&subscribe,&config);
    }

    while !queue.is_empty() {
        let  single_folder: &mut ReadDir = queue.back_mut().unwrap();

        let mut folder_founded:bool=false;

        for each_folder_element in single_folder {
            let item:DirEntry = each_folder_element.unwrap();
            let file_name = &item.file_name();
            let name =file_name.to_str().unwrap();

            if(stack_length == 1 ){
                how_much_completed +=1;
            }

            if(exclude.contains(name)){
                // count it so the hidden folders exculed also count in the main root

                // if(stack_length == 1 ){
                //     how_much_completed +=1;
                // }

                continue;
            }

            let metadata= item.metadata().unwrap();




            //println!("Progress: completed {:.2}% ({} / {})", progress, how_much_completed, root_files_count);
            //println!("Current : {} started to process", item.path().to_str().unwrap());

            if (metadata.is_file()) {

                destination_root.push(name);

                let dest_copy = destination_root.clone();
                let progress = (how_much_completed as f32 / root_files_count as f32) * 100.0;

                if (thread_allowed) {

                    subscribe.assign_work(Box::new(move || {
                        // Here is your file operation inside the closure
                        // here is the copy or move happens
                        file_operation(item.path(), dest_copy);

                        println!("Progress: completed {:.2}% ({} / {})", progress, how_much_completed, root_files_count);
                    }));

                }else {
                    file_operation(item.path(), dest_copy);
                    println!("Progress: completed {:.2}% ({} / {})", progress, how_much_completed, root_files_count);
                }


                destination_root.pop();

                // if(stack_length == 1 ){
                //     how_much_completed +=1;
                // }
            }else if  (metadata.is_dir())  {
                // if(stack_length == 1 ){
                //     how_much_completed +=1;
                // }
                folder_founded = true;

                queue.push_back(read_dir(item.path()).unwrap());


                destination_root.push(name);
                match create_dir_all(&destination_root) {
                    Ok(_) => {},
                    Err(e) => {},
                    // Ok(_) => println!("Created destination directory: {:?}", destination_root),
                    // Err(e) => eprintln!("Failed to create destination directory may present {:?}: {}", destination_root, e),
                }
                stack_length += 1;
                break;
            } else {
                println!("Skipping non-file/non-directory item: {}", name);
            }

        }

        /*
         If no subfolder was found in this iteration (`folder_founded == false`),
         it means the current folder is fully processed and can be removed from the queue.
         Otherwise, if a subfolder was found, it will be added to the queue in the directory
         handling block above, along with updating the destination path.

         When we're done with a folder:
          1. Remove it from the queue (go back to the previous folder in the stack).
          2. Pop the last folder name from `destination_root` so new files go
             to the correct parent destination.
          3. Decrement `stack_length` to track depth.
          4. Log that we’ve moved back one level.

          If this were a move operation, we could also delete the now-empty source folder
        */
        if(!folder_founded){
            // Remove the last folder from the queue so the next loop iteration
            // continues from the previous directory in the traversal.
            queue.pop_back().unwrap();

            // if(is_move){
            //     fs::remove_dir(current_file_processed_folder)
            // }

            // Move `destination_root` back to the previous level.
            // This avoids having to re-compute and convert the full
            // "from → to" path mapping every time.
            destination_root.pop();

            if(stack_length==1){
                let progress = if root_files_count > 0 {
                    (how_much_completed as f32 / root_files_count as f32) * 100.0
                } else {
                    0.0
                };
                println!("Progress: completed {:.2}% ({} / {})", progress, how_much_completed, root_files_count);
            }
            stack_length -= 1;
        }

    }

    subscribe.set_all_files_shared_status(true);

    if let Some(threads) = handler {
        for each_thread in threads {
            // wait for each thread to finish task
            each_thread.join().unwrap();
        }
    }




    println!("\nCopy process completed. Total files/directories processed: {}", how_much_completed);
    press_any_key_to_exit();
}

fn count(read_dir:ReadDir) -> u32 {
    let mut count = 0;

    for entry_result in read_dir {
        count += 1;
    }
    return  count;
}


fn copy_files(from: PathBuf, to: PathBuf)  {
    copy(&from,&to);
    println!("{:?}",from)
}

fn move_file(from: PathBuf, to: PathBuf)  {
    println!("Move is not implemented")
    // copy(&from, to);
    //
    // // deleted the copied file
    // fs::remove_file(&from);
}

fn create_thread(subscribe:&Subscribe ,config:&Config) -> Option<Vec<JoinHandle<()>>> {
    let mut handler:Vec<JoinHandle<_>> =vec![];

    let total_core_on_system=get_cpu_core();
    const FAST_DIVISION_HUNDRED:f64 = 1.0/100.0;
    /*
     calculate the core usage calculation via percentage
     percentage can user type from any number 0 to 1000
     so we need to make sure it is between 1 to 100
     here we clamp calculate the percentage via dividing that to 100
     so to divide by 100 we use multiplication way of that is
     by doing like this below
     1/100 gives fast  0.00000233304 some value multipling this
     with any value is equal to divinding with that thing

    */
    let clamp_percentage = if config.cpu_thread_usage_percent > 101 {
        (total_core_on_system as f64) * (100.0 * FAST_DIVISION_HUNDRED)
    } else if config.cpu_thread_usage_percent < 1 {
        (total_core_on_system as f64) * (10.0 * FAST_DIVISION_HUNDRED)
    } else {
        (total_core_on_system as f64) *  (config.cpu_thread_usage_percent as f64 * FAST_DIVISION_HUNDRED)
    };

    let final_core_allowed = clamp_percentage.ceil() as usize;

    println!("Threads usage for current task : {}", final_core_allowed);


    /*
    creating here thread and making it regular chek the que
    task added by main thread and  here these thread complete that
    */
    for id in 0..final_core_allowed {

        let subscribed:Subscribe = subscribe.duplicate_handle();

        handler.push(thread::spawn(move || {
            //let mut exit_count:u8=0;
            loop {
                // if(exit_count==200){
                //     break;
                // }
                // Try to get a task
                let job_opt = subscribed.get_work();

                match job_opt {
                    Some(job) => {
                        //exit_count=0;
                        // println!("Thread {id} got a job");
                        job(); // execute closure
                    }
                    None => {

                        if(subscribed.get_all_files_shared_status()){
                            break;
                        }else {
                            thread::sleep(Duration::from_millis(500));
                        }
                        // exit_count+=1;
                        // if(exit_count>5){
                        //     println!("Thread {id} going to end");
                        // }
                        // No job: sleep a bit so we don’t busy-spin
                    }
                }
            }
        }));
    }
    return Some(handler);
}

fn read_configuration()-> Config {
    let data = match fs::read_to_string("config.json") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading config.json: {}", e);
            press_any_key_to_exit();
            std::process::exit(1);
        }
    };


    let config: Config = serde_json::from_str(&data).unwrap_or_else(|e| {
        eprintln!("Error parsing JSON: {}", e);
        press_any_key_to_exit();
        std::process::exit(1);
    });
    return  config;
}


struct Subscribe {
    thread_sharable_storage_queue:Arc<Mutex<VecDeque<Box<dyn FnOnce() + Send>>>>,
    all_files_shared_to_queue:Arc<Mutex<bool>>
}


impl Subscribe {

    fn new()-> Subscribe {
        Subscribe{
            thread_sharable_storage_queue:Arc::new(Mutex::new(VecDeque::with_capacity(1000))),
            all_files_shared_to_queue:Arc::new(Mutex::new(false))
        }
    }
    fn assign_work(&self,closure:Box<dyn FnOnce() + Send>){
        let mut locked = self.thread_sharable_storage_queue.lock().unwrap();
        locked.push_back(closure);
        drop(locked);
    }

    fn duplicate_handle(&self)->Subscribe{
        Subscribe{
            all_files_shared_to_queue:Arc::clone(&self.all_files_shared_to_queue),
            thread_sharable_storage_queue:Arc::clone(&self.thread_sharable_storage_queue)
        }
    }

    fn get_work(&self)-> Option<Box<dyn FnOnce()+Send>> {
        let mut q = self.thread_sharable_storage_queue.lock().unwrap();
        q.pop_front()
    }

    fn set_all_files_shared_status(&self,value:bool){
        let mut status:MutexGuard<bool> = self.all_files_shared_to_queue.lock().unwrap();
        *status = value;
    }
    fn get_all_files_shared_status(&self)-> bool{
        let  status:MutexGuard<bool> = self.all_files_shared_to_queue.lock().unwrap();
        *status
    }

}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    process: String,
    source_path: String,
    destination_path: String,
    #[serde(default)]
    exclude: HashSet<String>,
    #[serde(default)]
    multi_threading:bool,
    #[serde(default)]
    cpu_thread_usage_percent:usize
}


fn press_any_key_to_exit(){
    println!("\nPress enter to exit!");
    let mut user_responce:String=String::new();
    stdin().read_line(&mut user_responce).unwrap();
}

 fn get_cpu_core()->usize{
    let cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    cores
}

// fn thread_way(dest:&PathBuf,subscribe:&Subscribe ,file_oper:&fn(from: PathBuf, to: PathBuf),item:DirEntry)  {
//     // file_oper();
// }
//
// fn syncronus_way(dest:&PathBuf,subscribe:&Subscribe ,file_oper:&fn(from: PathBuf, to: PathBuf),item:DirEntry)  {
//     println!("Move is not implemented")
// }