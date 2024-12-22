const CHRISTMAS_EMOJIS: [char; 4] = ['🎅', '🤶', '🎄', '🎁'];

pub trait Anonymize {
    fn anonymize_email(&self) -> String;
}

impl Anonymize for String {
    fn anonymize_email(&self) -> String {
        let parts: Vec<&str> = self.split('@').collect();

        if parts.len() == 2 {
            let email = parts[0];
            let domain = parts[1];

            let anonymous: String = email
                .chars()
                .enumerate()
                .map(|(i, _)| CHRISTMAS_EMOJIS[i % CHRISTMAS_EMOJIS.len()])
                .collect();

            format!("{}@{}", anonymous, domain)
        } else {
            self.chars()
                .enumerate()
                .map(|(i, _)| CHRISTMAS_EMOJIS[i % CHRISTMAS_EMOJIS.len()])
                .collect()
        }
    }
}

pub fn main() {
    let emails = vec![
        "rudolph.therapysessions@northpole.com".to_string(),
        "elfhr.complaint@northpole.urgent".to_string(),
        "santas.rage.management@christmaschaos.noel".to_string(),
        "overtimepay.never@elfexploitation.workshop".to_string(),
        "mrs.claus.divorce.lawyer@northpole.legal".to_string(),
        "reindeer.workers.comp@antler.insurance".to_string(),
        "naughty.list.revenge@santasecrets.com".to_string(),
        "workshop.ptsd.support@elves.anonymous".to_string(),
        "performance.anxiety@santa.breakdown".to_string(),
        "existential.crisis@northpole.void".to_string(),
    ];

    for email in emails {
        let anonymized_email = email.anonymize_email();
        println!("Original: {} -> Anonymized: {}", email, anonymized_email);
    }
}
