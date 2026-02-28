fn expanded_form(n: u64) -> String 
{
    n
    .to_string()
    .chars()
    .rev()
    .enumerate()
    .filter_map(|(i,c)| 
        {
            let digit = c.to_digit(10)?;
            if digit == 0 
            {
                None 
            } 
            else 
            {
                Some(format!("{}{}", digit, "0".repeat(i)))
            }
        })
    .collect::<Vec<String>>()
    .into_iter()
    .rev() 
    .collect::<Vec<String>>()
    .join(" + ")
}
