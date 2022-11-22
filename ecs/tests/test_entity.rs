
#[cfg(test)]
mod test_entity{
    use resa_ecs::entity::Entity;

    // #[derive(Component)]
    struct FakeComponent {}

    #[test]
    fn add_component_to_entity(){

        let mut entity = Entity::new(1);
        let fc = FakeComponent{};
        entity.add_component(fc);

        let asserted_components_length = 1;
        assert_eq!(asserted_components_length, entity.components.len());
    }
}