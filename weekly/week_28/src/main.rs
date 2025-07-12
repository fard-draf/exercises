use week_28::populate;
use week_28::day_5::ex1_synthese::*;

fn main(

  

) {
    let mut entity = HotData::new(SIZE_ENTITY_NUMBER);
  populate(SIZE_ENTITY_NUMBER, &mut entity);
  println!("{:#?}", size_of::<HotData>());

}

