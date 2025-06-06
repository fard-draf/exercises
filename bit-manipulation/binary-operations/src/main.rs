fn hex_to_binary_correspondence() {
    // Chaque chiffre hexadécimal = 4 bits
    println!("Hex | Binaire | Décimal");
    println!("----|---------|--------");
    println!("0x0 | 0b0000  | {}", 0x0);
    println!("0x7 | 0b0111  | {}", 0x7);
    println!("0xA | 0b1010  | {}", 0xA);
    println!("0xF | 0b1111  | {}", 0xF);

    // Pour un u32, nous avons besoin de 8 chiffres hex (32 bits / 4 = 8)
    let packed_data: u32 = 0x12345678;
    println!("\n0x12345678 se décompose ainsi :");
    println!("1    2    3    4    5    6    7    8");
    println!("0001 0010 0011 0100 0101 0110 0111 1000");

    // Les masques classiques en hexadécimal
    println!("\nMasques courants pour u32 :");
    println!("0x000000FF = {:032b} (dernier octet)", 0x000000FFu32);
    println!("0x0000FFFF = {:032b} (derniers 16 bits)", 0x0000FFFFu32);
    println!("0xFFFFFFFF = {:032b} (tous les bits)", 0xFFFFFFFFu32);
}

fn explore_type_sizes() {
    // Chaque type a une taille fixe en bits
    println!("u8  : {} bits, max = {}", 8, u8::MAX);
    println!("u16 : {} bits, max = {}", 16, u16::MAX);
    println!("u32 : {} bits, max = {}", 32, u32::MAX);
    println!("u64 : {} bits, max = {}", 64, u64::MAX);

    // Visualisons un u8 sous différentes formes
    let value: u8 = 171;
    println!("\nLe nombre {} s'écrit :", value);
    println!("Binaire      : 0b{:08b}", value);
    println!("Hexadécimal  : 0x{:02X}", value);
}

fn left_shift_deep_dive() {
    let value: u8 = 0b00000011; // 3 en décimal

    println!("Valeur initiale : {:08b} ({})", value, value);

    // Observer l'effet du décalage pas à pas
    for shift in 0..=7 {
        let shifted = value << shift;
        println!("  << {} : {:08b} ({:3})", shift, shifted, shifted);
    }

    // Application pratique : construire un masque
    fn create_mask_at_position(width: u32, position: u32) -> u32 {
        let base_mask = (1u32 << width) - 1; // Crée 'width' bits à 1
        base_mask << position // Les positionne
    }

    println!("\nCréation de masques avec décalage :");
    let mask = create_mask_at_position(4, 12);
    println!("4 bits à la position 12 : 0x{:08X} = {:032b}", mask, mask);
}

fn right_shift_extraction() {
    // Imaginons des données RGB packées : RRRRRGGG GGGBBBBB
    let packed_color: u16 = 0b1111100111100111;

    // Extraction du rouge (bits 11-15)
    let red = packed_color >> 11; // Ramène les bits rouges en position 0-4

    println!("Couleur packée : {:016b}", packed_color);
    println!("Après >> 11    : {:016b}", red);
    println!("Rouge extrait  : {} (sur 31 max)", red);

    // Piège à éviter : décalage de types signés
    let signed: i8 = -64; // 0b11000000 en complément à 2
    let shifted = signed >> 2;
    println!("\nAttention aux types signés :");
    println!("{} >> 2 = {} (extension du signe !)", signed, shifted);
}

fn and_operation_mastery() {
    println!("=== L'opération AND : principe du masquage ===\n");

    // Visualisation de l'opération AND
    let data = 0b11010110u8;
    let mask = 0b00111100u8;
    let result = data & mask;

    println!("  data   : {:08b}", data);
    println!("  mask   : {:08b}", mask);
    println!("  result : {:08b}", result);
    println!("\nSeuls les bits où mask=1 sont conservés !");

    // Application : vérifier des flags
    struct StatusFlags;
    impl StatusFlags {
        const READY: u8 = 0b00000001;
        const ACTIVE: u8 = 0b00000010;
        const ERROR: u8 = 0b00000100;
    }

    let status = 0b00000011u8; // READY et ACTIVE sont activés

    if status & StatusFlags::READY != 0 {
        println!("Le système est prêt");
    }
    if status & StatusFlags::ERROR == 0 {
        println!("Aucune erreur détectée");
    }
}

fn or_operation_building() {
    println!("=== L'opération OR : construction de données ===\n");

    // Construction progressive d'une valeur
    let mut packed_data: u32 = 0;

    // Ajout du premier champ (ID sur 8 bits)
    let id: u8 = 42;
    packed_data |= (id as u32) << 0;
    println!("Après ajout ID     : 0x{:08X}", packed_data);

    // Ajout du second champ (Type sur 4 bits à la position 8)
    let data_type: u8 = 7;
    packed_data |= ((data_type & 0xF) as u32) << 8;
    println!("Après ajout Type   : 0x{:08X}", packed_data);

    // Ajout du troisième champ (Flags sur 4 bits à la position 12)
    let flags: u8 = 0b1010;
    packed_data |= ((flags & 0xF) as u32) << 12;
    println!("Après ajout Flags  : 0x{:08X}", packed_data);

    println!("\nRésultat final en binaire :");
    println!("{:032b}", packed_data);
    println!("FFFFTTTTIIIIIIII (F=Flags, T=Type, I=ID)");
}
fn main() {
    or_operation_building();
}
