use std::collections::{HashMap, HashSet};
use crate::nfa::{NFA, StateId as NFAStateId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DFAStateId(pub usize);

#[derive(Debug, Clone)]
pub struct DFAState {
    pub nfa_states: HashMap<usize, HashSet<NFAStateId>>, // Map from NFA index to states
    pub is_accepting: bool,
    pub rule_index: Option<usize>, // Index of the matching rule (for precedence)
}

#[derive(Debug, Clone)]
pub struct DFA {
    pub states: HashMap<DFAStateId, DFAState>,
    pub start_state: DFAStateId,
    pub transitions: HashMap<(DFAStateId, char), DFAStateId>,
    next_state_id: usize,
}

impl DFA {
    pub fn from_nfas(nfas: Vec<(NFA, usize)>) -> Self {
        let mut dfa = DFA {
            states: HashMap::new(),
            start_state: DFAStateId(0),
            transitions: HashMap::new(),
            next_state_id: 0,
        };

        // Create start state with all NFA start states
        let mut start_nfa_states = HashMap::new();
        for (nfa_index, (nfa, _)) in nfas.iter().enumerate() {
            let mut start_set = HashSet::new();
            start_set.insert(nfa.start_state.clone());
            let epsilon_closure = nfa.epsilon_closure(&start_set);
            start_nfa_states.insert(nfa_index, epsilon_closure);
        }

        let start_state = dfa.new_state(start_nfa_states, &nfas);
        dfa.start_state = start_state;

        // Build DFA using subset construction
        let mut worklist = vec![dfa.start_state.clone()];
        let mut processed = HashSet::new();

        while let Some(current_state_id) = worklist.pop() {
            if processed.contains(&current_state_id) {
                continue;
            }
            processed.insert(current_state_id.clone());

            let current_state = dfa.states.get(&current_state_id).unwrap().clone();

            // For each possible input character
            for ch in (32..127u8).map(|b| b as char) {
                let mut next_nfa_states = HashMap::new();

                // Compute move on character for each NFA separately
                for (nfa_index, (nfa, _)) in nfas.iter().enumerate() {
                    if let Some(current_nfa_states) = current_state.nfa_states.get(&nfa_index) {
                        let moved = nfa.move_on_char(current_nfa_states, ch);
                        if !moved.is_empty() {
                            let epsilon_closure = nfa.epsilon_closure(&moved);
                            next_nfa_states.insert(nfa_index, epsilon_closure);
                        }
                    }
                }

                if !next_nfa_states.is_empty() {
                    // Find or create DFA state
                    let next_state_id = dfa.find_or_create_state(next_nfa_states, &nfas);

                    // Add transition
                    dfa.transitions.insert((current_state_id.clone(), ch), next_state_id.clone());

                    if !processed.contains(&next_state_id) {
                        worklist.push(next_state_id);
                    }
                }
            }
        }

        dfa
    }

    fn new_state(&mut self, nfa_states: HashMap<usize, HashSet<NFAStateId>>, nfas: &[(NFA, usize)]) -> DFAStateId {
        let state_id = DFAStateId(self.next_state_id);
        self.next_state_id += 1;

        let (is_accepting, rule_index) = check_accepting(&nfa_states, nfas);

        let state = DFAState {
            nfa_states,
            is_accepting,
            rule_index,
        };

        self.states.insert(state_id.clone(), state);
        state_id
    }

    fn find_or_create_state(&mut self, nfa_states: HashMap<usize, HashSet<NFAStateId>>, nfas: &[(NFA, usize)]) -> DFAStateId {
        // Check if state already exists
        for (state_id, state) in &self.states {
            if state.nfa_states == nfa_states {
                return state_id.clone();
            }
        }

        // Create new state
        self.new_state(nfa_states, nfas)
    }

    pub fn simulate(&self, input: &str) -> Vec<(String, usize, usize, Option<usize>)> {
        let mut tokens = Vec::new();
        let mut line = 1;
        let mut column = 1;
        let mut pos = 0;
        let chars: Vec<char> = input.chars().collect();

        while pos < chars.len() {
            let (token_length, rule_index) = self.longest_match(&chars[pos..]);

            if token_length > 0 {
                let lexeme: String = chars[pos..pos + token_length].iter().collect();
                tokens.push((lexeme, line, column, rule_index));

                // Update position
                for i in pos..pos + token_length {
                    if chars[i] == '\n' {
                        line += 1;
                        column = 1;
                    } else {
                        column += 1;
                    }
                }
                pos += token_length;
            } else {
                // No match found, skip character
                if chars[pos] == '\n' {
                    line += 1;
                    column = 1;
                } else {
                    column += 1;
                }
                pos += 1;
            }
        }

        tokens.push(("".to_string(), line, column, None)); // EOF marker
        tokens
    }

    fn longest_match(&self, input: &[char]) -> (usize, Option<usize>) {
        let mut current_state = &self.start_state;
        let mut last_accepting_pos = 0;
        let mut last_accepting_rule = None;

        // Check if start state is accepting
        if let Some(state) = self.states.get(current_state) {
            if state.is_accepting {
                last_accepting_pos = 0;
                last_accepting_rule = state.rule_index;
            }
        }

        for (pos, &ch) in input.iter().enumerate() {
            if let Some(next_state_id) = self.transitions.get(&(current_state.clone(), ch)) {
                current_state = next_state_id;

                if let Some(state) = self.states.get(current_state) {
                    if state.is_accepting {
                        last_accepting_pos = pos + 1;
                        last_accepting_rule = state.rule_index;
                    }
                }
            } else {
                break;
            }
        }

        (last_accepting_pos, last_accepting_rule)
    }
}

fn check_accepting(nfa_states: &HashMap<usize, HashSet<NFAStateId>>, nfas: &[(NFA, usize)]) -> (bool, Option<usize>) {
    let mut best_rule_index = None;

    for (nfa_index, (nfa, rule_index)) in nfas.iter().enumerate() {
        if let Some(current_nfa_states) = nfa_states.get(&nfa_index) {
            for accept_state in &nfa.accept_states {
                if current_nfa_states.contains(accept_state) {
                    // Found an accepting state - check if this rule has higher precedence
                    match best_rule_index {
                        None => best_rule_index = Some(*rule_index),
                        Some(current_best) => {
                            if *rule_index < current_best {
                                best_rule_index = Some(*rule_index);
                            }
                        }
                    }
                }
            }
        }
    }

    (best_rule_index.is_some(), best_rule_index)
}
