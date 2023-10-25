mod associativity;
mod precedence;

pub use associativity::{Associativity, AssociativityValue};
pub use precedence::{Precedence, PrecedenceValue};

pub struct EvaluationRules {
    associativity: AssociativityValue,
    precedence: PrecedenceValue,
}

impl EvaluationRules {
    pub fn is_left_associative(&self) -> bool {
        self.associativity.is_left()
    }

    pub fn is_right_associative(&self) -> bool {
        self.associativity.is_right()
    }

    pub fn associativity(&self) -> AssociativityValue {
        self.associativity
    }

    pub fn precedence(&self) -> PrecedenceValue {
        self.precedence
    }
}

pub trait EvaluationOrder {
    fn evaluation_rules(&self) -> EvaluationRules;

    fn should_wrap<T: EvaluationOrder>(&self, other: &T, is_associative: bool) -> bool {
        let self_rules = self.evaluation_rules();
        let other_rules = other.evaluation_rules();

        match self_rules.precedence().cmp(&other_rules.precedence()) {
            // If it has a lower precedence, it will always need to be wrapped.
            std::cmp::Ordering::Less => true,

            // If it has a higher precedence, it will never need to be wrapped.
            std::cmp::Ordering::Greater => false,

            // If it has the same precedence, we will need to wrap if the current operator is associative.
            std::cmp::Ordering::Equal => is_associative,
        }
    }

    fn should_wrap_left<T: EvaluationOrder>(&self, other: &T) -> bool {
        Self::should_wrap(self, other, self.evaluation_rules().is_left_associative())
    }

    fn should_wrap_right<T: EvaluationOrder>(&self, other: &T) -> bool {
        Self::should_wrap(self, other, self.evaluation_rules().is_right_associative())
    }
}

impl<T: Associativity + Precedence> EvaluationOrder for T {
    fn evaluation_rules(&self) -> EvaluationRules {
        EvaluationRules {
            associativity: self.associativity(),
            precedence: self.precedence(),
        }
    }
}
