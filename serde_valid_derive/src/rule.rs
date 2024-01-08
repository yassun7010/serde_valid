mod named_struct_rule;
mod struct_custom;
mod unnamed_struct_rule;

pub use named_struct_rule::collect_rules_from_named_struct;
pub use struct_custom::collect_struct_custom_from_named_struct;
pub use unnamed_struct_rule::collect_rules_from_unnamed_struct;
