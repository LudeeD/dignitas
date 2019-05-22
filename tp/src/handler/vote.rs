use sawtooth_sdk::processor::handler::ApplyError;

#[derive(Debug, Clone)]
pub struct Vote {
    vote_id: u32,
    agree: u32,
    disagree: u32,
}

impl Vote {
    pub fn new(vote_id: u32) -> Vote {
        Vote {
            vote_id,
            agree: 0,
            disagree: 0,
        }
    }

    pub fn to_string(&self) -> String {
        let fields = vec![
            self.vote_id.clone().to_string(),
            self.agree.clone().to_string(),
            self.disagree.clone().to_string(),
        ];
        fields.join(",")
    }

    pub fn from_string(vote_string: &str) -> Option<Vote> {
        let items: Vec<&str> = vote_string.split(',').collect();
        if items.len() != 3 {
            return None;
        }
        let g = Vote {
            vote_id: items[0]
                .to_string()
                .parse()
                .expect("Failed to Parse Vote From String"),
            agree: items[1]
                .to_string()
                .parse()
                .expect("Failed to Parse Vote From String"),
            disagree: items[2]
                .to_string()
                .parse()
                .expect("Failed to Parse Vote From String"),
        };
        Some(g)
    }

    pub fn agree_more(&mut self, value: u32) -> Result<(), ApplyError> {
        info!("Function agree_more : {}", value);
        self.agree = self.agree + value;
        Ok(())
    }

    pub fn disagree_more(&mut self, value: u32) -> Result<(), ApplyError> {
        info!("Function disagree_more : {}", value);
        self.disagree = self.disagree + value;
        Ok(())
    }
}
