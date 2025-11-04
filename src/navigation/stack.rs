use super::{NavigationAction, Route};

#[derive(Clone, Debug)]
pub struct NavigationStack {
    stack: Vec<Route>,
}

impl NavigationStack {
    pub fn new(initial_route: Route) -> Self {
        Self {
            stack: vec![initial_route],
        }
    }

    pub fn current(&self) -> &Route {
        self.stack
            .last()
            .expect("Navigation stack should never be empty")
    }

    pub fn current_mut(&mut self) -> &mut Route {
        self.stack
            .last_mut()
            .expect("Navigation stack should never be empty")
    }

    pub fn apply_action(&mut self, action: NavigationAction) {
        match action {
            NavigationAction::Push(route) => {
                self.stack.push(route);
            }
            NavigationAction::Pop => {
                if self.stack.len() > 1 {
                    self.stack.pop();
                }
            } // NavigationAction::Replace(route) => {
              //     if let Some(last) = self.stack.last_mut() {
              //         *last = route;
              //     }
              // }
              // NavigationAction::Reset(route) => {
              //     self.stack.clear();
              //     self.stack.push(route);
              // }
        }
    }

    pub fn can_go_back(&self) -> bool {
        self.stack.len() > 1
    }

    pub fn depth(&self) -> usize {
        self.stack.len()
    }
}

impl Default for NavigationStack {
    fn default() -> Self {
        Self::new(Route::home())
    }
}
