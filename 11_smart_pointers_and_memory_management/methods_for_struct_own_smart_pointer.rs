#[derive(Debug)]
struct ImportantData {
    id: u32,
    payload: String,
}

// This struct owns an ImportantData instance on the heap
struct DataOwner {
    name: String,
    // Box<T> is good when the struct needs to own T and T is large,
    // or T's size is unknown (like a trait object).
    data_ptr: Option<Box<ImportantData>>,
}

impl DataOwner {
    fn new(name: &str) -> Self {
        DataOwner {
            name: name.to_string(),
            data_ptr: None,
        }
    }

    fn initialize_data(&mut self, id: u32, payload: &str) {
        self.data_ptr = Some(Box::new(ImportantData {
            id,
            payload: payload.to_string(),
        }));

        println!("{} initialized data.", self.name);
    }

    fn update_payload(&mut self, new_payload: &str) {
        // self.data_ptr is an Option<Box<ImportantData>>
        // .as_mut() gives Option<&mut Box<ImportantData>>
        // .map() operates on the &mut Box if Some
        if let Some(boxed_data_mut_ref) = self.data_ptr.as_mut() {
            // boxed_data_mut_ref is &mut Box<ImportantData>
            // We can call methods on ImportantData thanks to DerefMut on Box
            // or directly assign to fields if they are public.
            // Here, `payload` is public on `ImportantData`.
            boxed_data_mut_ref.payload = new_payload.to_string();
            println!(
                "{} updated payload for ID {:?}.",
                self.name, boxed_data_mut_ref
            );
        } else {
            println!("{} has no data to update.", self.name);
        }
    }

    fn display_data(&self) {
        // self.data_ptr is an Option<Box<ImportantData>>
        // .as_ref() gives Option<&Box<ImportantData>>
        if let Some(boxed_data_ref) = self.data_ptr.as_ref() {
            // boxed_data_ref is &Box<ImportantData>
            // We can access fields of ImportantData thanks to Deref on Box
            println!(
                "{}'s Data: [ID: {:?}]: {}",
                self.name, boxed_data_ref, boxed_data_ref.payload
            );
        } else {
            println!("{} has no data to display", self.name);
        }
    }
}

fn main() {
    let mut owner1 = DataOwner::new("OwnerAlpha");
    owner1.display_data(); // No data
    owner1.initialize_data(101, "Initial crucial data");
    owner1.display_data(); // Initial crucial data
    owner1.update_payload("Updated important information");
    owner1.display_data(); // Updated important information
}
