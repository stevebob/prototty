#version 450

layout(location = 0) in vec3 a_BackgroundColour;
layout(location = 1) in vec3 a_ForegroundColour;
layout(location = 2) in int a_Underline;

layout(location = 0) flat out vec3 v_BackgroundColour;
layout(location = 1) flat out vec3 v_ForegroundColour;
// We only need the y component of this ratio, but opengl complains if this is a float.
// Error is: "initializer of type int cannot be assigned to variable of type float"
layout(location = 2) out vec2 v_CellRatio;
layout(location = 3) flat out int v_Underline;

layout(set = 0, binding = 0) uniform Globals {
    vec2 u_CellSizeRelativeToWindow;
    vec2 u_OffsetTtoCentre;
    int u_GridWidth;
};

out gl_PerVertex {
    vec4 gl_Position;
};

// a vec2 causes a shader compile error:
// "too many parameters to `vec2` constructor"
/*
const vec2 corner_offsets[6] = vec2[6](
    vec2(0.0, 0.0), // 0
    vec2(1.0, 0.0), // 1
    vec2(1.0, 1.0), // 2
    vec2(0.0, 0.0), // 3
    vec2(1.0, 1.0), // 4
    vec2(0.0, 1.0)  // 5
);*/

// Similarly, looking up each component of the coordinate individually and
// storing the result in floats causes the error:
// "too many parameters for float constructor"
/*
const float corner_offsets_x[6] = float[6](0, 1, 1, 0, 1, 0);
const float corner_offsets_y[6] = float[6](0, 0, 1, 0, 1, 1);
*/

void main() {
  // The magic numbers have the binary representation such that the shift and
  // mask operations are equivalent to looking up gl_VertexIndex in the tables
  // commented-out above.
  float corner_offset_x = float(22 >> gl_VertexIndex & 1);
  float corner_offset_y = 1.0 - float(52 >> gl_VertexIndex & 1);
  vec2 corner_offset = vec2(corner_offset_x, corner_offset_y);
  v_BackgroundColour = a_BackgroundColour;
  v_ForegroundColour = a_ForegroundColour;
  v_Underline = a_Underline;
  vec2 cell_size = u_CellSizeRelativeToWindow;
  uint grid_width = u_GridWidth;
  uint coord_x = gl_InstanceIndex % grid_width;
  uint coord_y = gl_InstanceIndex / grid_width;
  vec2 coord = vec2(coord_x, coord_y);
  vec2 scaled_corner_offset =  corner_offset * cell_size;
  vec2 top_left_corner = coord * cell_size;
  vec2 absolute = vec2(-1.0, -1.0) + top_left_corner + scaled_corner_offset + u_OffsetTtoCentre;
  absolute.y *= -1;
  v_CellRatio = corner_offset;
  gl_Position = vec4(absolute, 0.0, 1.0);
}
