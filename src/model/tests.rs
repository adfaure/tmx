use super::*;
use std::str::FromStr;

use ::error::Error;

#[test]
fn after_reading_valid_xml_expect_map_to_have_version() {
    let map = get_simple_valid_map();
    assert_eq!("1.0", map.version());
}

#[test]
fn after_reading_valid_xml_expect_map_to_have_orientation() {
    let map = get_simple_valid_map();
    assert_eq!(Orientation::Orthogonal, map.orientation());
}

#[test]
fn after_reading_valid_xml_without_render_order_expect_map_to_have_default_render_order() {
    let map = get_simple_valid_map();
    assert_eq!(RenderOrder::RightDown, map.render_order());
}

#[test]
fn after_reading_valid_xml_with_render_order_expect_map_to_have_that_render_order() {
    let map = Map::from_str("<map renderorder=\"left-up\"></map>").unwrap();
    assert_eq!(RenderOrder::LeftUp, map.render_order());
}

#[test]
fn after_reading_valid_xml_expect_map_to_have_width() {
    let map = get_simple_valid_map();
    assert_eq!(200, map.width());
}

#[test]
fn after_reading_valid_xml_expect_map_to_have_height() {
    let map = get_simple_valid_map();
    assert_eq!(100, map.height());
}

#[test]
fn after_reading_valid_xml_expect_map_to_have_tile_width() {
    let map = get_simple_valid_map();
    assert_eq!(16, map.tile_width());
}

#[test]
fn after_reading_valid_xml_expect_map_to_have_tile_height() {
    let map = get_simple_valid_map();
    assert_eq!(32, map.tile_height());
}

#[test]
fn after_reading_valid_xml_expect_map_to_have_next_object_id() {
    let map = get_simple_valid_map();
    assert_eq!(1, map.next_object_id());
}

#[test]
fn when_reading_map_xml_with_invalid_attribute_expect_attribute_error() {
    let result = Map::from_str(r#"<map bad=""></map>"#);
    assert_matches!(result, Err(Error::UnknownAttribute(..)));
}

#[test]
fn when_reading_map_xml_with_invalid_orientation_expect_orientation_error() {
    let result = Map::from_str(r#"<map orientation="bad"></map>"#);
    assert_matches!(result, Err(Error::BadOrientation(..)));
}

#[test]
fn when_reading_map_xml_with_invalid_render_order_expect_render_order_error() {
    let result = Map::from_str(r#"<map renderorder="bad"></map>"#);
    assert_matches!(result, Err(Error::BadRenderOrder(..)));
}

#[test]
fn when_reading_invalid_xml_element_expect_error() {
    let result = Map::from_str("<nomap/>");
    assert_matches!(result, Err(Error::BadXml));
}

#[test]
fn after_reading_xml_with_tilesets_expect_map_to_be_iterable_over_tilesets() {
    let map = get_map_with_tilesets();
    assert_eq!(2, map.tilesets().count());
}

#[test]
fn after_reading_xml_with_layers_expect_map_to_be_iterable_over_layers() {
    let map = get_map_with_layers();
    assert_eq!(5, map.layers().count());

    let mut layers = map.layers();
    let layer1 = layers.next().unwrap();
    assert_eq!("layer1_name", layer1.name());
    assert_eq!(1.0, layer1.opacity());
    assert!(layer1.is_visible());
    assert_eq!(0, layer1.offset_x());
    assert_eq!(0, layer1.offset_y());

    let layer2 = layers.next().unwrap();
    assert_eq!(0.0, layer2.opacity());

    let layer3 = layers.next().unwrap();
    assert!(!layer3.is_visible());

    let layer4 = layers.next().unwrap();
    assert_eq!(1, layer4.offset_x());
    assert_eq!(2, layer4.offset_y());

    let layer5 = layers.next().unwrap();
    assert_eq!(1, layer5.properties().count());
}

#[test]
fn after_reading_xml_with_image_layers_expect_map_to_be_iterable_over_image_layers() {
    let map = get_map_with_image_layers();
    assert_eq!(6, map.image_layers().count());

    let mut layers = map.image_layers();
    let layer1 = layers.next().unwrap();
    assert_eq!("layer1_name", layer1.name());
    assert_eq!(1.0, layer1.opacity());
    assert!(layer1.is_visible());
    assert_eq!(0, layer1.offset_x());
    assert_eq!(0, layer1.offset_y());

    let layer2 = layers.next().unwrap();
    assert_eq!(0.0, layer2.opacity());

    let layer3 = layers.next().unwrap();
    assert!(!layer3.is_visible());

    let layer4 = layers.next().unwrap();
    assert_eq!(1, layer4.offset_x());
    assert_eq!(2, layer4.offset_y());

    let layer5 = layers.next().unwrap();
    assert_eq!(1, layer5.properties().count());

    let layer6 = layers.next().unwrap();
    assert!(layer6.image().is_some());
}

#[test]
fn after_reading_xml_with_object_groups_expect_map_to_be_iterable_over_object_groups() {
    let map = get_map_with_objectgroups();
    assert_eq!(2, map.object_groups().count());

    let mut object_groups = map.object_groups();
    let group1 = object_groups.next().unwrap();
    assert_eq!("some_name", group1.name());
    assert_eq!(1.0, group1.opacity());
    assert!(group1.is_visible());
    assert_eq!(0, group1.offset_x());
    assert_eq!(0, group1.offset_y());
    assert_eq!(DrawOrder::TopDown, group1.draw_order());
    assert_eq!(1, group1.properties().count());

    let group2 = object_groups.next().unwrap();
    assert_eq!(0.0, group2.opacity());
    assert!(!group2.is_visible());
    assert_eq!(1, group2.offset_x());
    assert_eq!(2, group2.offset_y());
    assert_eq!(DrawOrder::Index, group2.draw_order());
}

#[test]
fn after_reading_valid_xml_expect_tileset_to_have_first_gid() {
    let tileset = get_simple_valid_tileset();
    assert_eq!(1, tileset.first_gid());
}

#[test]
fn after_reading_valid_xml_expect_tileset_to_have_name() {
    let tileset = get_simple_valid_tileset();
    assert_eq!("simple", tileset.name());
}

#[test]
fn after_reading_valid_xml_expect_tileset_to_have_tile_width() {
    let tileset = get_simple_valid_tileset();
    assert_eq!(32, tileset.tile_width());
}

#[test]
fn after_reading_valid_xml_expect_tileset_to_have_tile_height() {
    let tileset = get_simple_valid_tileset();
    assert_eq!(16, tileset.tile_height());
}

#[test]
fn after_reading_valid_xml_expect_tileset_to_have_tile_count() {
    let tileset = get_simple_valid_tileset();
    assert_eq!(100, tileset.tile_count());
}

#[test]
fn after_reading_valid_xml_with_image_element_expect_tileset_to_have_image() {
    let tileset = Tileset::from_str(
        r#"<tileset>
        <image source="some_file.png"
                width="1024"
                height="768"/>
    <tileset>"#).unwrap();
    let image = tileset.image().unwrap();
    assert_eq!("some_file.png", image.source());
    assert_eq!(1024, image.width());
    assert_eq!(768, image.height());
}

#[test]
fn after_reading_valid_xml_with_properties_expect_tileset_to_have_properties() {
    let tileset = Tileset::from_str(
        r#"<tileset>
        <properties>
            <property name="prop1_name" value="prop1_value"/>
            <property name="prop2_name" value="0" type="int"/>
            <property name="prop3_name" value="0.0" type="float"/>
            <property name="prop4_name" value="true" type="bool"/>
        </properties>
    <tileset>"#).unwrap();
    assert_eq!(4, tileset.properties().count());
    let mut props = tileset.properties();

    let prop1 = props.next().unwrap();
    assert_eq!("prop1_name", prop1.name());
    assert_eq!("prop1_value", prop1.value());
    assert_eq!(PropertyType::String, prop1.property_type());

    let prop2 = props.next().unwrap();
    assert_eq!(PropertyType::Int, prop2.property_type());

    let prop3 = props.next().unwrap();
    assert_eq!(PropertyType::Float, prop3.property_type());

    let prop4 = props.next().unwrap();
    assert_eq!(PropertyType::Bool, prop4.property_type());
}

#[test]
fn after_reading_valid_xml_with_tile_offset_expect_tileset_to_have_tile_offset() {
    let tileset = Tileset::from_str(
        r#"<tileset>
        <tileoffset x="0" y="1"/>
    <tileset>"#).unwrap();
    let offset = tileset.tile_offset().unwrap();
    assert_eq!(0, offset.x());
    assert_eq!(1, offset.y());
}

#[test]
fn after_reading_valid_xml_with_terrains_expect_tileset_to_have_terrains() {
    let tileset = Tileset::from_str(r#"
    <tileset>
        <terraintypes>
            <terrain name="terrain1"/>
            <terrain tile="tile-id">
                <properties>
                    <property/>
                </properties>
            </terrain>
        </terraintypes>
    <tileset>"#).unwrap();
    assert_eq!(2, tileset.terrain_types().count());
    let mut terrain_types = tileset.terrain_types();

    let terrain1 = terrain_types.next().unwrap();
    assert_eq!("terrain1", terrain1.name());

    let terrain2 = terrain_types.next().unwrap();
    assert_eq!("tile-id", terrain2.tile());
    assert_eq!(1, terrain2.properties().count());
}

fn get_simple_valid_map() -> Map {
    Map::from_str(r#"<map version="1.0"
        orientation="orthogonal"
        width="200"
        height="100"
        tilewidth="16"
        tileheight="32"
        nextobjectid="1">
    </map>"#).unwrap()
}

fn get_map_with_tilesets() -> Map {
    Map::from_str("<map>
        <tileset></tileset>
        <tileset></tileset>
    </map>").unwrap()
}

fn get_map_with_layers() -> Map {
    Map::from_str(r#"<map>
        <layer name="layer1_name"/>
        <layer name="layer2_name" opacity="0"/>
        <layer name="layer3_name" visibility="0"/>
        <layer name="layer4_name" offsetx="1" offsety="2"/>
        <layer>
            <properties>
                <property name="some_name" value="some_value"/>
            </properties>
        </layer>
    </map>"#).unwrap()
}

fn get_map_with_image_layers() -> Map {
    Map::from_str(r#"<map>
        <imagelayer name="layer1_name"/>
        <imagelayer name="layer2_name" opacity="0"/>
        <imagelayer name="layer3_name" visibility="0"/>
        <imagelayer name="layer4_name" offsetx="1" offsety="2"/>
        <imagelayer>
            <properties>
                <property name="some_name" value="some_value"/>
            </properties>
        </imagelayer>
        <imagelayer>
            <image source="some_file.png"
                    width="1024"
                    height="768"/>
        </imagelayer>
    </map>"#).unwrap()
}

fn get_simple_valid_tileset() -> Tileset {
    Tileset::from_str(r#"<tileset firstgid="1"
                name="simple"
                tilewidth="32"
                tileheight="16"
                tilecount="100">
    </tileset>"#).unwrap()
}

fn get_map_with_objectgroups() -> Map {
    Map::from_str(r#"<map>
        <objectgroup name="some_name">
            <properties>
                <property/>
            </properties>
        </objectgroup>
        <objectgroup opacity="0" visibility="0" offsetx="1" offsety="2" draworder="index">
        </objectgroup>
    </map>"#).unwrap()
}

