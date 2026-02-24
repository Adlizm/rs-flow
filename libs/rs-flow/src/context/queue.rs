use std::collections::VecDeque;

pub(crate) enum ReceiveQueue<P> {
    Closed,
    Open(VecDeque<P>),
}
impl<P> ReceiveQueue<P> {
    pub fn new() -> Self {
        Self::Open(VecDeque::new())
    }

    pub fn close(&mut self) {
        *self = Self::Closed
    }

    pub fn push_all(&mut self, packages: &mut VecDeque<P>) {
        match self {
            Self::Open(queue) => queue.append(packages),
            Self::Closed => {}
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Open(queue) => queue.is_empty(),
            Self::Closed => true,
        }
    }

    pub fn get_next(&mut self) -> Option<P> {
        match self {
            Self::Open(queue) => queue.pop_front(),
            Self::Closed => None,
        }
    }

    pub fn get_all(&mut self) -> Vec<P> {
        match self {
            Self::Open(queue) => {
                let mut packages = VecDeque::<P>::new();
                std::mem::swap(queue, &mut packages);

                packages.into()
            }
            Self::Closed => Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Open(queue) => queue.len(),
            Self::Closed => 0,
        }
    }
}
