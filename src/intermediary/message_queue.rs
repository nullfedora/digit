
///Stores a FIFO queue of messages intended for communicating between threads
pub struct MessageQueue<T>{
    messages: Vec<T>
}

impl<T> MessageQueue<T>{
    ///Creates a new empty message queue
    pub fn new() -> MessageQueue<T>{
        MessageQueue{
            messages: Vec::new()
        }
    }

    ///Get the number of messages currently in the queue.
    pub fn len(&self) -> usize{
        self.messages.len()
    }

    ///Add a new message to the end of the queue.
    pub fn add_message(&mut self, new_message: T){
        self.messages.push(new_message);
    }

    ///Gets the oldest message in the queue, also removing that message from the queue.
    pub fn get_message(&mut self) -> T{
        self.messages.remove(0)
    }

    ///Gets whether the queue has messages in it
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

///Messages that the frontend thread can send to the backend thread
pub enum FrontendMessage{
    UserQuit,
    TestMessage
}

///Messages that the backend thread can send to the frontend thread
pub enum BackendMessage{
    TestMessage
}

#[cfg(test)]
mod tests{
    
    use super::*;

    #[test]
    fn new_message_queue_has_length_zero(){
        let x: MessageQueue<()> = MessageQueue::new();

        assert_eq!(x.len(), 0);
    }

    #[test]
    fn adding_message_has_length_one(){
        let mut x: MessageQueue<()> = MessageQueue::new();

        x.add_message(());

        assert_eq!(x.len(), 1);
    }

    #[test]
    fn removing_message_returns_correct_message(){
        let mut x: MessageQueue<i32> = MessageQueue::new();

        x.add_message(1);
        x.add_message(22);

        assert_eq!(x.get_message(), 1);
    }

    #[test]
    fn removing_message_has_length_zero(){
        let mut x: MessageQueue<i32> = MessageQueue::new();

        x.add_message(1);

        x.get_message();

        assert_eq!(x.len(), 0);
    }

    #[test]
    fn new_message_queue_is_empty(){
        let x: MessageQueue<()> = MessageQueue::new();

        assert_eq!(x.is_empty(), true);
    }

    #[test]
    fn message_queue_with_message_is_not_empty(){
        let mut x: MessageQueue<()> = MessageQueue::new();

        x.add_message(());

        assert_eq!(x.is_empty(), false);
    }
}