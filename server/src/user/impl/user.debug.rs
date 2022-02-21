use crate::user::User;
use std::fmt;

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("current_ct_id", &self.current_ct_id.clone())
            .field("current_pt_id", &self.current_pt_id.clone())
            .finish()
    }
}
