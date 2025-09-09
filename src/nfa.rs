use std::collections::{HashMap, HashSet};
use crate::regex_parser::RegexNode;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StateId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Transition {
    Char(char),
    Epsilon,
}

#[derive(Debug, Clone)]
pub struct NFA {
    pub states: HashSet<StateId>,
    pub start_state: StateId,
    pub accept_states: HashSet<StateId>,
    pub transitions: HashMap<(StateId, Transition), HashSet<StateId>>,
    next_state_id: usize,
}

impl NFA {
    pub fn new() -> Self {
        Self {
            states: HashSet::new(),
            start_state: StateId(0),
            accept_states: HashSet::new(),
            transitions: HashMap::new(),
            next_state_id: 0,
        }
    }

    fn new_state(&mut self) -> StateId {
        let state = StateId(self.next_state_id);
        self.next_state_id += 1;
        self.states.insert(state.clone());
        state
    }

    fn add_transition(&mut self, from: StateId, transition: Transition, to: StateId) {
        self.transitions
            .entry((from, transition))
            .or_insert_with(HashSet::new)
            .insert(to);
    }

    pub fn from_regex(regex: &RegexNode) -> Self {
        let mut nfa = NFA::new();
        let start = nfa.new_state();
        let accept = nfa.new_state();

        nfa.start_state = start.clone();
        nfa.accept_states.insert(accept.clone());

        nfa.build_nfa(regex, start, accept);
        nfa
    }

    fn build_nfa(&mut self, regex: &RegexNode, start: StateId, accept: StateId) {
        match regex {
            RegexNode::Char(ch) => {
                self.add_transition(start, Transition::Char(*ch), accept);
            }
            RegexNode::Dot => {
                // Match any character except newline
                for ch in (32..127u8).map(|b| b as char) {
                    if ch != '\n' {
                        self.add_transition(start.clone(), Transition::Char(ch), accept.clone());
                    }
                }
            }
            RegexNode::Concatenation(left, right) => {
                let middle = self.new_state();
                self.build_nfa(left, start, middle.clone());
                self.build_nfa(right, middle, accept);
            }
            RegexNode::Alternation(left, right) => {
                self.build_nfa(left, start.clone(), accept.clone());
                self.build_nfa(right, start, accept);
            }
            RegexNode::Kleene(inner) => {
                // ε-transition from start to accept (zero matches)
                self.add_transition(start.clone(), Transition::Epsilon, accept.clone());

                // Create loop for one or more matches
                let loop_start = self.new_state();
                let loop_end = self.new_state();

                self.add_transition(start, Transition::Epsilon, loop_start.clone());
                self.build_nfa(inner, loop_start.clone(), loop_end.clone());
                self.add_transition(loop_end.clone(), Transition::Epsilon, accept);
                self.add_transition(loop_end, Transition::Epsilon, loop_start);
            }
            RegexNode::Plus(inner) => {
                // One or more: equivalent to inner followed by inner*
                let middle = self.new_state();
                self.build_nfa(inner, start, middle.clone());

                // Add Kleene closure part
                let loop_start = self.new_state();
                self.add_transition(middle.clone(), Transition::Epsilon, loop_start.clone());
                self.add_transition(middle, Transition::Epsilon, accept.clone());

                let loop_end = self.new_state();
                self.build_nfa(inner, loop_start.clone(), loop_end.clone());
                self.add_transition(loop_end.clone(), Transition::Epsilon, accept);
                self.add_transition(loop_end, Transition::Epsilon, loop_start);
            }
            RegexNode::Optional(inner) => {
                // Zero or one: ε-transition to accept (zero) or through inner (one)
                self.add_transition(start.clone(), Transition::Epsilon, accept.clone());
                self.build_nfa(inner, start, accept);
            }
            RegexNode::CharClass(chars) => {
                for &ch in chars {
                    self.add_transition(start.clone(), Transition::Char(ch), accept.clone());
                }
            }
            RegexNode::NegatedCharClass(chars) => {
                let excluded: HashSet<char> = chars.iter().cloned().collect();
                for ch in (32..127u8).map(|b| b as char) {
                    if !excluded.contains(&ch) && ch != '\n' {
                        self.add_transition(start.clone(), Transition::Char(ch), accept.clone());
                    }
                }
            }
        }
    }

    pub fn epsilon_closure(&self, states: &HashSet<StateId>) -> HashSet<StateId> {
        let mut closure = states.clone();
        let mut stack: Vec<StateId> = states.iter().cloned().collect();

        while let Some(state) = stack.pop() {
            if let Some(epsilon_targets) = self.transitions.get(&(state, Transition::Epsilon)) {
                for target in epsilon_targets {
                    if !closure.contains(target) {
                        closure.insert(target.clone());
                        stack.push(target.clone());
                    }
                }
            }
        }

        closure
    }

    pub fn move_on_char(&self, states: &HashSet<StateId>, ch: char) -> HashSet<StateId> {
        let mut result = HashSet::new();

        for state in states {
            if let Some(targets) = self.transitions.get(&(state.clone(), Transition::Char(ch))) {
                result.extend(targets.iter().cloned());
            }
        }

        result
    }
}
