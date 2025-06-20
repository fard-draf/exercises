use std::{iter::Peekable, path::Iter};

pub struct RunIterator<I: Iterator> {
    actual: Peekable<I>    
}

impl<I: Iterator + Clone> RunIterator<I> {
    fn new(start: I) -> Self {
        Self { actual: start.peekable() }
    }

}

impl<I> Iterator for RunIterator<I> 
    where 
        I: Iterator,
        I::Item: PartialEq + Clone,

{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> 
    {
        
    match self.actual.peek() {
        
    
        Some(first_item_ref) => {
            let run_value = first_item_ref.clone();
            let mut actual_run = Vec::<I::Item>::new();

            while let Some(next_item_ref) = self.actual.peek() {
                if *next_item_ref == run_value {
                    actual_run.push(self.actual.next().unwrap());
                } else {
                    break;
                }
            }

            if actual_run.is_empty() {
                None
            } else {
                Some(actual_run)
                
            }

        }
        
        None => None,
    }
    }

}

