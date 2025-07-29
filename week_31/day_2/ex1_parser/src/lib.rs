#[derive(Debug, PartialEq, Default)]
pub struct ParsedMessage<'a> {
    pub message: &'a [u8],
    pub payload: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub enum ParseErr {
    InvalidFormat,
    MissingPayload,
}

pub fn parse<'a>(buffer: &'a [u8]) -> Result<ParsedMessage<'a>, ParseErr> {
    if buffer.is_empty() {
        return Err(ParseErr::InvalidFormat);
    }
    if let Ok((Some(val1), Some(val2))) =
        buffer
            .iter()
            .enumerate()
            .try_fold((None, None), |mut acc_pos, (index, &bytes)| {
                if index == 0 && bytes != b'$' {
                    return Err(ParseErr::InvalidFormat);
                }
                if bytes == b',' && acc_pos.0.is_none() {
                    acc_pos.0 = Some(index);
                }
                if bytes == b'*' {
                    acc_pos.1 = Some(index);
                }
                Ok(acc_pos)
            })
    {
        Ok(ParsedMessage {
            message: &buffer[1..val1],
            payload: &buffer[val1 + 1..val2],
        })
    } else {
        return Err(ParseErr::MissingPayload);
    }
}
