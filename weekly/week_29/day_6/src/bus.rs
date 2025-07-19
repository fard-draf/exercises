// Contexte no_std, donc on utilise heapless.
// use heapless::Vec as HeapVec;

#[derive(Debug)]
pub struct Device {
    pub id: u8,
    // Des données internes au périphérique
    pub data: heapless::Vec<u8, 16usize>,
}

#[derive(Debug)]
pub struct Bus {
    // Le bus possède deux périphériques
    pub devices: heapless::Vec<Device, 2usize>,
}

impl Bus {
    pub fn run_interaction<F>(
        &mut self,
        id_dev1: u8,
        id_dev2: u8,
        interaction: F,
    ) -> Result<(), &'static str>
    where
        F: FnOnce(&mut Device, &mut Device),
    {
        if id_dev1 == id_dev2 {
            return Err("Identicals ID");
        }

        let id_1 = self.devices.iter().position(|e| e.id == id_dev1);
        let id_2 = self.devices.iter().position(|e| e.id == id_dev2);

        if let (Some(id1), Some(id2)) = (id_1, id_2) {
            if id1 == id2 {
                return Err("Identicals ID");
            }

            // on defini l ordre des index du plus petit au plus grand, afin de rester coherant et de
            // s assurer du bon fonctionnement de la suite

            let (i, j) = if id1 < id2 { (id1, id2) } else { (id2, id1) };

            // ici, nous creons deux slices distinctes, permettant au llvm de savoir precisement que nous ne travaillons
            // pas sur la meme section memoire.

            let (left_slice, right_slice) = self.devices.split_at_mut(j);

            let mut dev_1 = &mut left_slice[i];
            let mut dev_2 = &mut right_slice[0];

            interaction(&mut dev_1, &mut dev_2);
            Ok(())
        } else {
            Err("Device not found")
        }
    }
}
