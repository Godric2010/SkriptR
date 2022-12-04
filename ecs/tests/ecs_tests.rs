
#[cfg(test)]
mod ecs_tests{
    use std::borrow::Borrow;
    use std::ops::Index;
    use resa_ecs::world::World;

    struct Demo{val: i32}

    #[test]
    fn create_entities(){

        let mut world = World::new();
        let entity01 = world.new_entity();
        let entity02 = world.new_entity();

        assert_eq!(entity01, 0);
        assert_eq!(entity02, 1);
    }

    #[test]
    fn create_and_borrow_component(){
        let mut world = World::new();
        let entity01 = world.new_entity();

        let demo = Demo{val: 42};

        world.add_component(entity01, demo);
        // let borrowed_demo = world.borrow_component_from_entity::<Demo>(entity01).as_ref();
        let borrowed_components = world.borrow_component_vec_mut::<Demo>().unwrap();
        // assert!(borrowed_components.is_some());
        let borrowed_demo = borrowed_components.iter().enumerate();
        let mut d: Option<&Demo> = None;
        for demo in borrowed_demo{
            if demo.0 == entity01{
                d = demo.1.as_ref()
            }
        }

        assert!(d.is_some());
        assert_eq!(d.unwrap().val, 42)

    }

}