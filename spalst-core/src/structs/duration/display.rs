//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Duration;

impl Duration {
    pub fn display(&self) -> String {
        if self.secs_total() == 0 {
            return "less than a second".to_string();
        };
        for field in Self::fields().iter().rev() {
            // No need to check for "nanoseconds" because the loop will end on "seconds" anyway.
            if *field == "seconds" {
                return format!(
                    "{} second{}",
                    self.seconds(),
                    if *self.seconds() == 1 { "" } else { "s" }
                );
            };
            let field_value: &u64 = self.get_field(field).unwrap();
            if *field_value > 0 {
                let secondary_field: &str = Self::fields()
                    .iter()
                    .rev()
                    .skip_while(|s: &&&str| -> bool { **s != *field })
                    .nth(1)
                    .unwrap();
                let secondary_field_value: &u64 = self.get_field(secondary_field).unwrap();
                if *secondary_field_value > 0 {
                    return format!(
                        "{} {}{} and {} {}{}",
                        field_value,
                        field,
                        if *field_value == 1 { "" } else { "s" },
                        secondary_field_value,
                        secondary_field,
                        if *secondary_field_value == 1 { "" } else { "s" }
                    );
                } else {
                    return format!(
                        "{} {}{}",
                        field_value,
                        field,
                        if *field_value == 1 { "" } else { "s" }
                    );
                };
            };
        }
        // Unreachable because of the if clause checking "seconds" inside the for loop. So the for
        // loop will always end on "seconds".
        unreachable!()
    }
}
