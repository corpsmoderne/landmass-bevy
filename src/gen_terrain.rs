use bevy::prelude::*;
use bevy::render::mesh::*;
use std::collections::HashMap;
use bevy::math::Vec3;
use crate::landmass::Landmass;

type Vectors = Vec<[f32;3]>;
type Uvs = Vec<[f32;2]>;
type NormMap = HashMap<(u32, u32), Vec3>;
type Colors = Vec<[f32;4]>;

pub fn gen_mesh_terrain(landmass: &Landmass, scale: f32) -> Mesh {
    let (vertices, _colors, uv0) = gen_vertices(landmass, scale);    
    let (triangles, norm_map) = gen_triangles(&vertices, landmass.size);
    let normals = gen_normals(&norm_map, landmass.size);
    
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    //mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv0);
    mesh.set_indices(Some(Indices::U32(triangles)));
    mesh
}

fn gen_vertices(landmass: &Landmass, scale: f32) -> (Vectors, Colors, Uvs) {
    let mut vertices = vec![];
    let mut uv0 = vec![];
    let mut colors = vec![];
    
    for z in 0..landmass.size {
	for x in 0..landmass.size {
	    let y = landmass.data[x as usize + (z*landmass.size) as usize];
	    let color = landmass.img[(x as u32, z as u32)];
	    vertices.push([x as f32 * scale,
			   y as f32 * 0.25 * scale,
			   z as f32 * scale]);
	    colors.push([1.0, 1.0, 1.0,
			 if y == 0.0 { 0.0 } else { 1.0 }]);
	    uv0.push([ (x as f32 / (landmass.size-1) as f32),
			(z as f32 / (landmass.size-1) as f32) ]);
		       
	}
    }
    (vertices, colors, uv0)
}

fn gen_triangles(vertices: &[[f32;3]], size: u32) -> (Vec<u32>, NormMap) {
    let mut triangles : Vec<u32> = vec![];
    let mut norm_map = NormMap::new(); 
    
    for y in 0..size-1 {
	for x in 0..size-1 {
	    let v1 = x + y * size;
	    let v2 = x + 1 + y * size;
	    let v3 = x + (y+1) * size;
	    let v4 = x + 1 + (y+1) * size;
	    
	    let vv1 = Vec3::from_array(vertices[v1 as usize]);
	    let vv2 = Vec3::from_array(vertices[v2 as usize]);
	    let vv3 = Vec3::from_array(vertices[v3 as usize]);
	    let vv4 = Vec3::from_array(vertices[v4 as usize]);
	    
	    let vd1 = vv2 - vv1;
	    let vd2 = vv3 - vv1;
	    let vnorm = vd2.cross(vd1);
	    norm_map.insert((x,y), vnorm);

	    //if vv1.y > 0.0 || vv2.y > 0.0 || vv3.y > 0.0 || vv4.y > 0.0 {
		triangles.push(v1);
		triangles.push(v4);
		triangles.push(v2);
		
		triangles.push(v1);
		triangles.push(v3);
		triangles.push(v4);
	    //}
	}
    }
    (triangles, norm_map)
}

fn gen_normals(norm_map: &NormMap, size: u32) -> Vectors {
    let mut normals : Vectors = vec![];
    
    for y in 0..size {
	for x in 0..size {
	    let mut v = Vec3::new(0.0, 0.0, 0.0);
	    if x < size-1 && y < size-1 {
		v += norm_map[&(x, y)];
	    }
	    if x > 0 && y > 0 {
		v += norm_map[&(x-1, y-1)];
	    }
	    if y > 0 && x < size-1 {
		v += norm_map[&(x, y-1)];
	    }
	    if x > 0 && y < size-1 {
		v += norm_map[&(x-1, y)];
	    }
	    normals.push(v.normalize().into());
	}
    }
    normals
}
