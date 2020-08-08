use std::sync::mpsc; // uses library to allow the library and the main file to message each other to make the server work with multiple processes
use std::sync::Arc; // uses library to make sure the threads used in the server are carefully maintained 
use std::sync::Mutex; // uses library to protect the communication between each file between communicating
use std::thread; // uses library to use the threads or the server device 

pub struct ThreadPool { // produces a structure for the thread pool variable 
    workers: Vec<Worker>, // makes a vector using the stucture variable Worker 
    sender: mpsc::Sender<Message>, // sends messages to terminal on how many threads a being used at once 
}

type Job = Box<dyn FnOnce() + Send + 'static>; // the variable Job is equal to the pointer in the stack where trait object types are called meaning it calls these into a variable

enum Message { // makes new predefined contant Message Structure
    NewJob(Job), // makes new job, meaning a job for a thread 
    Terminate, // stops the job when job is complete
}

impl ThreadPool { // Using structure Threadpool, and give meaning when talking about workers and sender
    pub fn new(size: usize) -> ThreadPool { // making a new data type called size allocating it to the structure of threadpool
        assert!(size > 0); // makes sure the amount of threads used is more than 0

        let (sender, receiver) = mpsc::channel(); // creates two variables sender and receiver to define which one is recieving and sending, and make a stream channel for data between the two files

        let receiver = Arc::new(Mutex::new(receiver)); // makes a secure connection to the reciever 

        let mut workers = Vec::with_capacity(size); // makes the borrowed workers variable keep in the size variables vector 

        for id in 0..size { // if the variable of id is 0 and size is bigger than 0
            workers.push(Worker::new(id, Arc::clone(&receiver))); // creates a new operation for a thread 
        }

        ThreadPool { workers, sender } // then sends the two variables workers and sender to the structure ThreadPool
    }

    pub fn execute<F>(&self, f: F) // produces a function that executes a callable function for Job that can create a job for a thread 
    where
        F: FnOnce() + Send + 'static, // creates the heap of the processes created 
    {
        let job = Box::new(f); // creates a pointer called f in the pool 

        self.sender.send(Message::NewJob(job)).unwrap(); // then it will allocate it as a new process or job for the threads 
    }
}

impl Drop for ThreadPool { // creating a smooth switching off process for the server using a drop function 
    fn drop(&mut self) { // creating a function called drop which find the address of the borrowed drop 
        println!("Sending terminate message to all workers."); // asking all processes to terminate 

        for _ in &self.workers { // finds all processes working in the lib file 
            self.sender.send(Message::Terminate).unwrap(); // and sends to the lib file to terminate all processes and executing it
        }

        println!("Shutting down all workers."); // letting server owner to know about it 

        for worker in &mut self.workers { // if the process as been completed by one of the workers in the lib file and its standing idle of a thread
            println!("Shutting down worker {}", worker.id); // telling server owner that one of the workers are shutting down to free up space for allocated threads

            if let Some(thread) = worker.thread.take() { // a thread is being used by a process 
                thread.join().unwrap(); // it will remove itself from that thread 
            }
        }
    }
}

struct Worker { // makes a structure called Worker 
    id: usize, // detais what id means by the structure 
    thread: Option<thread::JoinHandle<()>>, // allows to have access to the threads used 
}

impl Worker { // showing the functionality of the structure Worker 
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker { // creates new function which underlines the id variable and creates a secure message to the reciever and refer it back to the structure called Worker
        let thread = thread::spawn(move || loop { // makes a new variable called thread which creates a process for a thread detailed 
            let message = receiver.lock().unwrap().recv().unwrap(); // creates a new varible called message which allows the reciever file to hold the file and executes it

            match message { // if message finds this variable above to be used 
                Message::NewJob(job) => { // creates a new process for the thread using the message variable indication and use the job variable 
                    println!("Worker {} got a job; executing.", id); // tells owner of server that a process is being carried out 

                    job(); // then refers the infomation from the message and places it into the job varibale again 
                }
                Message::Terminate => { // if message is needing to terminate
                    println!("Worker {} was told to terminate.", id); // tells owner of server that a process was told to terminate 

                    break; // then breaks the cycle 
                }
            }
        });

        Worker {  // returns the results of the functionality of the Worker to the structure of the worker with id and thread variables 
            id,
            thread: Some(thread),
        }
    }
}