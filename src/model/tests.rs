// This file is part of tmx
// Copyright 2017 Sébastien Watteau
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::str::FromStr;

use error::Error;
use model::color::*;
use model::map::*;
use model::property::*;
use model::shape::*;
use model::tileset::*;

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
fn after_reading_hexagonal_map_xml_expect_map_to_have_special_attributes() {
    let map = Map::from_str("<map/>").unwrap();
    assert_eq!(None, map.hex_side_length());
    assert_eq!(None, map.stagger_axis());
    assert_eq!(None, map.stagger_index());

    let map = get_hexagonal_map();
    assert_eq!(Some(32), map.hex_side_length());
    assert_eq!(Some(Axis::Y), map.stagger_axis());
}

#[test]
fn after_reading_valid_xml_expect_map_to_have_next_object_id() {
    let map = get_simple_valid_map();
    assert_eq!(1, map.next_object_id());
}

#[test]
fn after_reading_valid_xml_with_background_color_expect_map_to_have_that_background_color() {
    let map = Map::from_str("<map/>").unwrap();
    assert_eq!(None, map.background_color());
    let map = Map::from_str("<map backgroundcolor=\"#80a0b0c0\"></map>").unwrap();
    assert_eq!(Some(&Color(128, 160, 176, 192)), map.background_color());
}

#[test]
fn after_reading_valid_xml_with_properties_expect_map_to_have_properties() {
    let map = Map::from_str(r#"<map>
        <properties>
            <property name="prop1_name" value="prop1_value"/>
            <property name="prop2_name" value="0" type="int"/>
            <property name="prop3_name" value="0.0" type="float"/>
            <property name="prop4_name" value="true" type="bool"/>
        </properties>
    </map>"#).unwrap();
    assert_eq!(4, map.properties().count());
}

#[test]
fn when_reading_map_xml_with_invalid_attribute_expect_attribute_error() {
    let result = Map::from_str(r#"<map bad=""></map>"#);
    assert_matches!(result, Err(Error::UnknownAttribute(..)));
}

#[test]
fn when_reading_map_xml_with_invalid_staggeraxis_expect_axis_error() {
    let result = Map::from_str(r#"<map staggeraxis="bad"></map>"#);
    assert_matches!(result, Err(Error::BadAxis(..)));
}

#[test]
fn when_reading_map_xml_with_invalid_staggerindex_expect_index_error() {
    let result = Map::from_str(r#"<map staggerindex="bad"></map>"#);
    assert_matches!(result, Err(Error::BadIndex(..)));
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
fn when_reading_map_xml_with_invalid_background_color_expect_invalid_color_error() {
    let result = Map::from_str(r#"<map backgroundcolor="bad"/>"#);
    assert_matches!(result, Err(Error::InvalidColor(..)));
}

#[test]
fn after_reading_xml_with_tilesets_expect_map_to_be_iterable_over_tilesets() {
    let map = get_map_with_tilesets();
    assert_eq!(2, map.tilesets().count());
}

#[test]
fn after_reading_xml_with_layers_expect_map_to_be_iterable_over_layers() {
    let map = get_map_with_layers();
    assert_eq!(7, map.layers().count());

    let mut layers = map.layers();
    let layer1 = layers.next().unwrap();
    assert_eq!("layer1_name", layer1.name());
    assert_eq!(1.0, layer1.opacity());
    assert!(layer1.is_visible());
    assert_eq!(0, layer1.offset_x());
    assert_eq!(0, layer1.offset_y());
    assert_eq!(0, layer1.x());
    assert_eq!(0, layer1.y());

    let layer2 = layers.next().unwrap();
    assert_eq!(0.0, layer2.opacity());

    let layer3 = layers.next().unwrap();
    assert!(!layer3.is_visible());

    let layer4 = layers.next().unwrap();
    assert_eq!(1, layer4.offset_x());
    assert_eq!(2, layer4.offset_y());
    assert_eq!(3, layer4.x());
    assert_eq!(4, layer4.y());
    assert_eq!(5, layer4.width());
    assert_eq!(6, layer4.height());

    let layer5 = layers.next().unwrap();
    assert_eq!(1, layer5.properties().count());

    let layer6 = layers.next().unwrap();
    let data = layer6.data().unwrap();
    assert_eq!(3, data.tiles().count());
    assert_eq!(None, data.encoding());
    assert_eq!(None, data.compression());
    assert_eq!(None, data.raw_content());

    let layer7 = layers.next().unwrap();
    let data = layer7.data().unwrap();
    assert_eq!(Some("base64"), data.encoding());
    assert_eq!(Some("gzip"), data.compression());
    assert_eq!(Some("SOME_ENCODED_AND_COMPRESSED_DATA"), data.raw_content());
}

#[test]
fn after_reading_xml_with_image_layers_expect_map_to_be_iterable_over_image_layers() {
    let map = get_map_with_image_layers();
    assert_eq!(7, map.image_layers().count());

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

    let layer7 = layers.next().unwrap();
    assert_eq!(1, layer7.x());
    assert_eq!(2, layer7.y());
    assert_eq!(3, layer7.width());
    assert_eq!(4, layer7.height());
}

#[test]
fn after_reading_xml_with_object_groups_expect_map_to_be_iterable_over_object_groups() {
    let map = get_map_with_objectgroups();
    assert_eq!(3, map.object_groups().count());

    let mut object_groups = map.object_groups();
    let group1 = object_groups.next().unwrap();
    assert_eq!("some_name", group1.name());
    assert_eq!(1.0, group1.opacity());
    assert!(group1.is_visible());
    assert_eq!(0, group1.offset_x());
    assert_eq!(0, group1.offset_y());
    assert_eq!(0, group1.x());
    assert_eq!(0, group1.y());
    assert_eq!(0, group1.width());
    assert_eq!(0, group1.height());
    assert_eq!(DrawOrder::TopDown, group1.draw_order());
    assert_eq!(1, group1.properties().count());

    let group2 = object_groups.next().unwrap();
    assert_eq!(0.0, group2.opacity());
    assert!(!group2.is_visible());
    assert_eq!(1, group2.offset_x());
    assert_eq!(2, group2.offset_y());
    assert_eq!(3, group2.x());
    assert_eq!(4, group2.y());
    assert_eq!(5, group2.width());
    assert_eq!(6, group2.height());
    assert_eq!(DrawOrder::Index, group2.draw_order());
    assert_eq!(None, group2.color());

    let group3 = object_groups.next().unwrap();
    assert_eq!(Some(&Color(255, 0, 0, 0)), group3.color());
}

#[test]
fn after_reading_xml_with_objects_expect_object_groups_to_be_iterable_over_objects() {
    let map = get_map_with_objects();
    let group = map.object_groups().next().unwrap();
    assert_eq!(6, group.objects().count());

    let mut objects = group.objects();

    let object = objects.next().unwrap();
    assert!(object.is_visible());
    assert_eq!(None, object.gid());
    assert_eq!(0, object.properties().count());
    assert_eq!(None, object.shape());

    let object = objects.next().unwrap();
    assert_eq!(1, object.id());
    assert_eq!("obj", object.name());
    assert_eq!("ty", object.object_type());
    assert_eq!(1.0, object.x());
    assert_eq!(2.0, object.y());
    assert_eq!(3.0, object.width());
    assert_eq!(4.0, object.height());
    assert_eq!(0.707, object.rotation());
    assert!(!object.is_visible());
    assert_eq!(Some(123), object.gid());

    let object = objects.next().unwrap();
    assert_eq!(1, object.properties().count());

    let object = objects.next().unwrap();
    assert_matches!(object.shape(), Some(&Shape::Ellipse));

    let object = objects.next().unwrap();
    assert_matches!(object.shape(), Some(&Shape::Polygon(..)));
    if let Some(&Shape::Polygon(ref polygon)) = object.shape() {
        assert_eq!(3, polygon.points().count());
        let mut points = polygon.points();
        assert_eq!(&Point {x: 0, y: 1}, points.next().unwrap());
        assert_eq!(&Point {x: 2, y: 3}, points.next().unwrap());
        assert_eq!(&Point {x: 4, y: 5}, points.next().unwrap());
    }

    let object = objects.next().unwrap();
    assert_matches!(object.shape(), Some(&Shape::Polyline(..)));
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
fn after_reading_valid_xml_expect_tileset_to_have_spacing() {
    let tileset = get_simple_valid_tileset();
    assert_eq!(4, tileset.spacing());
}

#[test]
fn after_reading_valid_xml_expect_tileset_to_have_margin() {
    let tileset = get_simple_valid_tileset();
    assert_eq!(2, tileset.margin());
}

#[test]
fn after_reading_valid_xml_expect_tileset_to_have_tile_count() {
    let tileset = get_simple_valid_tileset();
    assert_eq!(100, tileset.tile_count());
}

#[test]
fn after_reading_valid_xml_expect_tileset_to_have_source() {
    let tileset = get_simple_valid_tileset();
    assert_eq!("some_file.tsx", tileset.source());
}

#[test]
fn after_reading_valid_xml_expect_tileset_to_have_columns() {
    let tileset = get_simple_valid_tileset();
    assert_eq!(24, tileset.columns());
}

#[test]
fn after_reading_valid_xml_with_image_element_expect_tileset_to_have_image() {
    let tileset = Tileset::from_str(
        r#"<tileset>
        <image format="png"
               source="some_file.png"
               trans="FF00FF"
               width="1024"
               height="768">
            <data encoding="base64" compression="gzip"></data>
        </image>
    <tileset>"#).unwrap();
    let image = tileset.image().unwrap();
    assert_eq!("png", image.format());
    assert_eq!("some_file.png", image.source());
    assert_eq!(Some(&Color(255, 255, 0, 255)), image.trans());
    assert_eq!(1024, image.width());
    assert_eq!(768, image.height());
    assert!(image.data().is_some());
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

#[test]
fn after_reading_valid_xml_with_tiles_expect_tileset_to_be_iterable_over_tiles() {
    let tileset = Tileset::from_str(r#"
    <tileset>
        <tile id="123">
            <properties>
                <property name="some_name" value="some_value"/>
            </properties>
        </tile>
        <tile>
            <image source="some_file.png" width="8" height="16"/>
        </tile>
        <tile>
            <objectgroup/>
        </tile>
        <tile>
            <animation>
                <frame tileid="123" duration="500"/>
            </animation>
        </tile>
        <tile probability="0.5"/>
        <tile terrain="0,1,2,3"/>
    </tileset>"#).unwrap();
    assert_eq!(6, tileset.tiles().count());

    let tile1 = tileset.tiles().next().unwrap();
    assert_eq!(123, tile1.id());
    assert_eq!(1, tile1.properties().count());

    let tile2 = tileset.tiles().nth(1).unwrap();
    assert!(tile2.image().is_some());

    let tile3 = tileset.tiles().nth(2).unwrap();
    assert!(tile3.object_group().is_some());

    let tile4 = tileset.tiles().nth(3).unwrap();
    assert!(tile4.animation().is_some());
    let animation = tile4.animation().unwrap();
    assert!(animation.frame().is_some());
    let frame = animation.frame().unwrap();
    assert_eq!(123, frame.tile_id());
    assert_eq!(500, frame.duration());

    let tile5 = tileset.tiles().nth(4).unwrap();
    assert_eq!(Some(0.5), tile5.probability());

    let tile6 = tileset.tiles().nth(5).unwrap();
    assert_eq!(Some(&Corners(0, 1, 2, 3)), tile6.terrain());
}

#[test]
fn expect_point_to_be_constructible_from_comma_separated_pair_of_ints() {
    assert_matches!(Point::from_str(""), Err(Error::InvalidPoint(..)));
    assert_matches!(Point::from_str("1,2"), Ok(Point {x: 1, y: 2}));
    assert_matches!(Point::from_str("a,2"), Err(Error::InvalidNumber(..)));
    assert_matches!(Point::from_str("1,b"), Err(Error::InvalidNumber(..)));
    assert_matches!(Point::from_str("1,2,3"), Err(Error::InvalidPoint(..)));
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
        <layer name="layer3_name" visible="0"/>
        <layer name="layer4_name" offsetx="1" offsety="2" x="3" y="4" width="5" height="6"/>
        <layer>
            <properties>
                <property name="some_name" value="some_value"/>
            </properties>
        </layer>
        <layer>
            <data>
                <tile gid="1"/>
                <tile gid="2"/>
                <tile gid="3"/>
            </data>
        </layer>
        <layer>
            <data encoding="base64" compression="gzip">SOME_ENCODED_AND_COMPRESSED_DATA</data>
        </layer>
    </map>"#).unwrap()
}

fn get_map_with_image_layers() -> Map {
    Map::from_str(r#"<map>
        <imagelayer name="layer1_name"/>
        <imagelayer name="layer2_name" opacity="0"/>
        <imagelayer name="layer3_name" visible="0"/>
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
        <imagelayer x="1" y="2" width="3" height="4">
        </imagelayer>
    </map>"#).unwrap()
}

fn get_simple_valid_tileset() -> Tileset {
    Tileset::from_str(r#"<tileset firstgid="1"
                name="simple"
                source="some_file.tsx"
                tilewidth="32"
                tileheight="16"
                spacing="4"
                margin="2"
                tilecount="100"
                columns="24">
    </tileset>"#).unwrap()
}

fn get_map_with_objectgroups() -> Map {
    Map::from_str("<map>
        <objectgroup name=\"some_name\">
            <properties>
                <property/>
            </properties>
        </objectgroup>
        <objectgroup opacity=\"0\" visible=\"0\" draworder=\"index\"
            offsetx=\"1\" offsety=\"2\" x=\"3\" y=\"4\" width=\"5\" height=\"6\">
        </objectgroup>
        <objectgroup color=\"#ff000000\"/>
    </map>").unwrap()
}

fn get_map_with_objects() -> Map {
    Map::from_str(r#"<map>
        <objectgroup>
            <object/>
            <object id="1" name="obj" type="ty"
                    x="1" y="2" width="3" height="4"
                    rotation="0.707" visible="0"
                    gid="123"/>
            <object>
                <properties>
                    <property name="prop1" value="val1"/>
                </properties>
            </object>
            <object>
                <ellipse/>
            </object>
            <object>
                <polygon points="0,1 2,3 4,5"/>
            </object>
            <object>
                <polyline points="0,1 2,3 4,5"/>
            </object>
        </objectgroup>
    </map>"#).unwrap()
}

fn get_hexagonal_map() -> Map {
    Map::from_str(r#"<map orientation="hexagonal" hexsidelength="32"
        staggeraxis="y" staggerindex="even"/>"#).unwrap()
}

