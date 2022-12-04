
#[cfg(test)]
mod ecs_tests{
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

    #[test]
    fn remove_component(){
        let mut world = World::new();
        let entity01 = world.new_entity();

        let demo = Demo{val: 42};

        world.add_component(entity01, demo);
        world.remove_component::<Demo>(entity01);

        let demo_vec = world.borrow_component_vec_mut::<Demo>().unwrap();

        let borrowed_demos = demo_vec.iter().enumerate();
        for borrowed_demo in borrowed_demos{
            if borrowed_demo.0 == entity01{
                let demo = borrowed_demo.1.as_ref();
                assert!(demo.is_none())
            }
        }

    }

    #[test]
    fn remove_entity(){
        let mut world = World::new();
        let entity01 = world.new_entity();
        let entity02 = world.new_entity();

        let demo01 = Demo{val: 13};
        let demo02 = Demo{val: 42};

        world.add_component(entity01, demo01);
        world.add_component(entity02, demo02);

        world.remove_entity(entity01);

        let demo_vec = world.borrow_component_vec_mut::<Demo>().unwrap();
        assert_eq!(demo_vec.len(), 2);

        let borrowed_demos = demo_vec.iter().enumerate();
        for borrowed_demo in borrowed_demos{
            match borrowed_demo.0 {
                0 => assert!(borrowed_demo.1.as_ref().is_none()),
                1 => assert!(borrowed_demo.1.as_ref().is_some()),
                _=> panic!()
            }
        }
    }

}