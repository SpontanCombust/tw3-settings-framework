#[derive(Debug)]
pub enum VarType {
    Toggle,
    Options,
    SliderInt,
    SliderFloat
    // SubtleSeparator not included as it's just a cosmetic var
}

impl VarType {
    pub fn from_display_type(display_type: &str) -> Result<Option<VarType>, String> {
        if display_type == "TOGGLE" {
            return Ok(Some(VarType::Toggle));
        }
        if display_type == "OPTIONS" {
            return Ok(Some(VarType::Options));
        }
        if &display_type[0..6] == "SLIDER" {
            let spl: Vec<&str> = display_type.split(';').collect();

            if spl.len() == 1 {
                return Err("No slider parameters given".to_string());
            } 
            else if spl.len() != 4 {
                return Err(format!("Invalid amount of slider parameters. Should be 3, is {}", spl.len() - 1));
            } 
            else {
                //CHECKME min & max may need to be checked for being floats first

                let min = spl[1].parse::<i32>();
                if min.is_err() {
                    return Err(format!("Slider min value parse error: {}", min.unwrap_err()));   
                }

                let min = min.unwrap();


                let max = spl[2].parse::<i32>();
                if max.is_err() {
                    return Err(format!("Slider max value parse error: {}", max.unwrap_err()));   
                }

                let max = max.unwrap();


                let div = spl[3].parse::<i32>();
                if div.is_err() {
                    return Err(format!("Slider div value parse error: {}", div.unwrap_err()));   
                }

                let div = div.unwrap();
                if div <= 0 {
                    return Err("Slider div value must be greater than 0".to_string());
                }


                if min >= max {
                    return Err(format!("Slider min value is greater than max value: {}", min));
                }

                if (max - min) % div == 0 {
                    return Ok(Some(VarType::SliderInt))
                } 
                else {
                    return Ok(Some(VarType::SliderFloat))
                }
            }
        }
        if display_type == "SUBTLE_SEPARATOR" {
            return Ok(None);
        }

        return Err(format!("Unsupported display type: {}", display_type));
    }
}