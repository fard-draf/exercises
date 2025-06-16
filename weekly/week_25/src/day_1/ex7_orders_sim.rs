#[derive(Debug, PartialEq)]
enum OrderError {
    InvalidFormat(usize),    // Numéro de ligne (1-indexed)
    InvalidProductId(usize), // Numéro de ligne
    InvalidQuantity(String), // Product ID
    InvalidPrice(String),    // Product ID
    TotalTooLarge,
}

#[derive(Debug, Default)]
struct OrderItem {
    product_id: String,
    quantity: u32,
    price: f64,
}

fn process_orders(commands: &[&str]) -> Result<f64, OrderError> {
    // 🎯 Utiliser try_fold pour traiter et accumuler en une seule passe
    // 💡 Penser à l'ordre des validations et à la gestion des erreurs

    commands
        .iter()
        .enumerate()
        .try_fold(0.0, |mut acc, (line_number, line_str)| {
            let line_number = line_number + 1;
            let command = parse_order_line(line_str, line_number)
                .map_err(|_| OrderError::InvalidFormat(line_number))?;

            if command.product_id.is_empty() {
                return Err(OrderError::InvalidProductId(line_number));
            }

            if !(0.01..9999.99).contains(&command.price) {
                return Err(OrderError::InvalidPrice(command.product_id));
            }

            if command.quantity == 0 {
                return Err(OrderError::InvalidQuantity(command.product_id));
            }

            let line_total = command.quantity as f64 * command.price;
            let new_total = acc + line_total;

            if new_total > 1_000_000.0 {
                return Err(OrderError::TotalTooLarge);
            }

            Ok(new_total)
        })
}

// 🔧 Fonction helper pour parser une ligne
fn parse_order_line(line: &str, line_number: usize) -> Result<OrderItem, OrderError> {
    let mut splitted = line.split(':');
    // let line_number = line_number + 1;

    let product_id = splitted
        .next()
        .ok_or(OrderError::InvalidFormat(line_number))?
        .to_string();
    let raw_quantity = splitted
        .next()
        .ok_or(OrderError::InvalidFormat(line_number))?
        .parse::<u32>()
        .map_err(|_| OrderError::InvalidFormat(line_number))?;
    let price = splitted
        .next()
        .ok_or(OrderError::InvalidFormat(line_number))?
        .parse::<f64>()
        .map_err(|_| OrderError::InvalidFormat(line_number))?;

    if splitted.next().is_some() {
        return Err(OrderError::InvalidFormat(line_number));
    }

    let quantity = if raw_quantity <= 999 {
        raw_quantity
    } else {
        return Err(OrderError::InvalidFormat(line_number));
    };


    Ok(OrderItem {
        product_id,
        quantity,
        price,
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_orders() {
        let commands = ["ITEM1:2:10.50", "ITEM2:1:5.25"];
        assert_eq!(process_orders(&commands), Ok(26.25));

        let single = ["PROD:3:15.00"];
        assert_eq!(process_orders(&single), Ok(45.00));
    }

    #[test]
    fn test_format_errors() {
        assert_eq!(
            process_orders(&["ITEM1:2"]),
            Err(OrderError::InvalidFormat(1))
        );

        assert_eq!(
            process_orders(&["ITEM1:2:10.50", "INVALID"]),
            Err(OrderError::InvalidFormat(2))
        );
    }

    #[test]
    fn test_business_errors() {
        assert_eq!(
            process_orders(&["ITEM1:0:10.50"]),
            Err(OrderError::InvalidQuantity("ITEM1".to_string()))
        );

        assert_eq!(
            process_orders(&[":1:10.50"]),
            Err(OrderError::InvalidProductId(1))
        );

        assert_eq!(
            process_orders(&["ITEM1:2:-5.0"]),
            Err(OrderError::InvalidPrice("ITEM1".to_string()))
        );
    }

    #[test]
    fn test_overflow() {
        assert_eq!(
            process_orders(&["EXPENSIVE:200:5000.01"]),
            Err(OrderError::TotalTooLarge)
        );
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(process_orders(&[]), Ok(0.0));
    }

    #[test]
    fn test_stop_at_first_error() {
        // Première ligne invalide, ne doit pas traiter la suite
        let commands = ["INVALID", "ITEM1:1:10.0"];
        assert_eq!(process_orders(&commands), Err(OrderError::InvalidFormat(1)));
    }
}
