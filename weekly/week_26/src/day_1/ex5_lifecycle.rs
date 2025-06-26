#[derive(Debug, PartialEq)]
pub struct Resource {
    pub durability: u32,
}

#[derive(Debug)]
pub enum Task {
    Simple,
    Complex(u32),
}

pub struct ResourceProcessor {
    resource: Option<Resource>,
}

impl ResourceProcessor {
    pub fn new() -> Self {
        ResourceProcessor { resource: None }
    }

    pub fn load_resource(&mut self, resource: Resource) {
        self.resource = Some(resource);
    }

    pub fn process_batch(&mut self, tasks: Vec<Task>) -> Result<u32, &'static str> {
        let total_tasks = tasks.len() as u32;
        let mut resource = self.resource.take().ok_or("Processor has no resource")?;

        let mut tasks_completed = 0;

        for task in tasks {
            let cost = match task {
                Task::Simple => 10,
                Task::Complex(value) => value,
            };

            if resource.durability >= cost {
                resource.durability -= cost;
                tasks_completed += 1;
            } else {
                break;
            }
        }

        if resource.durability > 0 && tasks_completed == total_tasks {
            self.resource = Some(resource)
        }

        Ok(tasks_completed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_with_enough_durability() {
        let mut processor = ResourceProcessor::new();
        processor.load_resource(Resource { durability: 100 });
        let tasks = vec![Task::Simple, Task::Complex(50), Task::Simple];

        let result = processor.process_batch(tasks);
        assert_eq!(result, Ok(3));

        assert_eq!(processor.resource, Some(Resource { durability: 30 }));
    }

    #[test]
    fn test_process_until_resource_breaks() {
        let mut processor = ResourceProcessor::new();
        processor.load_resource(Resource { durability: 40 });
        let tasks = vec![Task::Complex(25), Task::Simple, Task::Simple]; // Total cost: 45

        let result = processor.process_batch(tasks);
        assert_eq!(result, Ok(2));

        assert_eq!(processor.resource, None);
    }

    #[test]
    fn test_process_with_no_resource() {
        let mut processor = ResourceProcessor::new();
        let tasks = vec![Task::Simple];
        let result = processor.process_batch(tasks);
        assert_eq!(result, Err("Processor has no resource"));
    }

    #[test]
    fn test_process_empty_batch() {
        let mut processor = ResourceProcessor::new();
        processor.load_resource(Resource { durability: 100 });
        let tasks = vec![];
        let result = processor.process_batch(tasks);
        assert_eq!(result, Ok(0));
        assert_eq!(processor.resource, Some(Resource { durability: 100 }));
    }
}
