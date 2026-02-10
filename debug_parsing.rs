use regex::Regex;

fn main() {
    let js_text = r#"
        var ARRFUTURESNODES = {
            czce: [['PTA', 'PTA', 'pta_qh'], ['SR', 'SR', 'sr_qh']],
            shfe: [['氧化铝', 'AO', 'ao_qh']],
        };
    "#;

    let start = js_text.find("ARRFUTURESNODES = {");
    let end = js_text.find("};");

    if start.is_none() || end.is_none() {
        println!("Start or end not found");
        return;
    }

    let content = &js_text[start.unwrap()..end.unwrap() + 2];
    println!("Content: {}", content);

    let exchanges = vec![("czce", "郑州商品交易所"), ("shfe", "上海期货交易所")];

    let item_re = Regex::new(r"\['([^']+)',\s*'([^']+)',\s*'[^']*'").unwrap();

    for (exchange_code, exchange_name) in exchanges {
        let pattern = format!(r"{}\s*:\s*\[", exchange_code);
        let re = Regex::new(&pattern).unwrap();

        if let Some(m) = re.find(content) {
            let start_pos = m.end();
            let remaining = &content[start_pos..];
            println!(
                "Found exchange {}. Remaining starts with: {}",
                exchange_code,
                &remaining[..20.min(remaining.len())]
            );

            let mut balance = 1;
            let mut end_pos = 0;

            for (i, c) in remaining.char_indices() {
                if c == '[' {
                    balance += 1;
                } else if c == ']' {
                    balance -= 1;
                }

                if balance == 0 {
                    end_pos = i;
                    println!("Found end of array at index {}, char: {}", i, c);
                    break;
                }
            }

            if end_pos > 0 {
                let array_content = &remaining[..end_pos];
                println!("Array content: {}", array_content);

                let mut count = 0;
                for cap in item_re.captures_iter(array_content) {
                    count += 1;
                    println!(
                        "  Matched: {:?} -> {:?}",
                        cap.get(1).map(|m| m.as_str()),
                        cap.get(2).map(|m| m.as_str())
                    );
                }
                println!("Total matches for {}: {}", exchange_code, count);
            } else {
                println!("Balance never reached 0 for {}", exchange_code);
            }
        } else {
            println!("Exchange {} not found", exchange_code);
        }
    }
}
