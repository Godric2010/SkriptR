#[cfg(test)]
mod ecs_tests {
    use resa_ecs::world::World;

    struct Demo {
        val: u32,
    }

    struct Demo2{
        val: String,
    }

    #[test]
    fn check_component_data() {
        let mut world = World::new();
        let entity_a = world.new_entity();
        let entity_b = world.new_entity();
        let entity_c = world.new_entity();

        let demo_a = Demo { val: 42 };
        let demo_b = Demo { val: 24 };
        let demo_c = Demo { val: 100};
        let demo_c2 = Demo2{ val: "Nothing".to_string()};

        world.add_component(&entity_a, demo_a);
        world.add_component(&entity_b, demo_b);
        world.add_component(&entity_c, demo_c);
        world.add_component(&entity_c, demo_c2);

        let received_demo_a = world.get_component::<Demo>(&entity_a).unwrap();
        let received_demo_b = world.get_component::<Demo>(&entity_b).unwrap();
        let received_demo_c = world.get_component::<Demo>(&entity_c).unwrap();

        assert_eq!(received_demo_a.val, 42);
        assert_eq!(received_demo_b.val, 24);
        assert_eq!(received_demo_c.val, 100);
    }

    #[test]
    fn change_component_data() {
        let mut world = World::new();
        let entity_a = world.new_entity();
        let entity_b = world.new_entity();

        let demo_a = Demo { val: 42 };
        let demo_b = Demo { val: 24 };

        world.add_component(&entity_a, demo_a);
        world.add_component(&entity_b, demo_b);

        let mut received_demo_a = world.get_component_mut::<Demo>(&entity_a).unwrap();

        received_demo_a.val = 110;

        let test_demo_a = world.get_component::<Demo>(&entity_a).unwrap();

        assert_eq!(test_demo_a.val, 110);
    }

    #[test]
    fn get_all_components_of_type(){
        let mut world = World::new();
        let entity_a = world.new_entity();
        let entity_b = world.new_entity();
        let entity_c = world.new_entity();

        let demo_a = Demo{val: 42};
        let demo_b = Demo2{val: "Test01".to_string()};
        let demo_c = Demo{val: 25};
        let demo_c2 = Demo2{val: "Test02".to_string()};

        world.add_component(&entity_a, demo_a);
        world.add_component(&entity_b, demo_b);
        world.add_component(&entity_c, demo_c);
        world.add_component(&entity_c, demo_c2);

        let demo_result = world.get_all_components_of_type::<Demo>().unwrap();
        let demo2_result = world.get_all_components_of_type::<Demo2>().unwrap();

        assert_eq!(demo_result.len(),2);
        assert_eq!(demo_result[0].0.val, 42);
        assert_eq!(demo_result[0].1, entity_a);
        assert_eq!(demo_result[1].0.val, 25);
        assert_eq!(demo_result[1].1, entity_c);

        assert_eq!(demo2_result.len(), 2);
        assert!(demo2_result[0].0.val == "Test01".to_string() && demo2_result[1].0.val == "Test02".to_string());
    }
}