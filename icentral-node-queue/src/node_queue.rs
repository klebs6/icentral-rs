crate::ix!();

///-----------------
#[derive(Debug)]
pub struct NodeIdQueue {
    name: String,
    data: Queue<NodeId>,
}

impl Named for NodeIdQueue {

    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }
}

impl CreateNamedEmpty for NodeIdQueue {

    fn empty(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            data: Queue::default(),
        }
    }
}

impl NodeIdQueue {

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn peek(&mut self) -> Option<NodeId> {

        debug!("peeking the NodeIdQueue {} of len: {}", self.name, self.len());

        self.data.peek()
    }

    pub fn is_empty(&self) -> bool {

        let is_empty = self.data.is_empty();

        debug!("checking whether the NodeIdQueue {} is empty? {}", self.name, is_empty);

        is_empty
    }

    pub fn new(first: NodeId, name: &str) -> Self {

        debug!("creating new NodeIdQueue named {} starting with node {}", name, first);

        let mut queue = NodeIdQueue::empty(name);

        queue.enqueue(first);

        queue
    }

    pub fn enqueue(&mut self, n: NodeId) {

        debug!("enqueuing NodeId {} into NodeIdQueue {}", n, self.name);

        self.data.queue(n);
    }

    pub fn dequeue(&mut self) -> Option<NodeId> {

        let maybe_node = self.data.dequeue();

        if let Some(node) = maybe_node {

            debug!("dequeued NodeId {:?} from NodeIdQueue {}", node, self.name);

        } else {

            debug!("attempted dequeue of NodeIdQueue {}, but there are no more nodes left on the queue!", self.name);
        }

        maybe_node
    }
}
