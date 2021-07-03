#version 450

layout(location = 0) out vec4 outColor;

layout(location = 0) flat in vec3 v_BackgroundColour;
layout(location = 1) flat in vec3 v_ForegroundColour;
layout(location = 2) in vec2 v_CellRatio;
layout(location = 3) flat in int v_Underline;

layout(set = 0, binding = 1) uniform Underline {
  float u_UnderlineWidthCellRatio;
  float u_UnderlineTopOffsetCellRatio;
};

void main() {
  if (v_Underline != 0 &&
      (1 - v_CellRatio.y) >= u_UnderlineTopOffsetCellRatio &&
      (1 - v_CellRatio.y) <= (u_UnderlineTopOffsetCellRatio + u_UnderlineWidthCellRatio)
  ) {
    outColor = vec4(v_ForegroundColour, 1.0);
  } else {
    outColor = vec4(v_BackgroundColour, 1.0);
  }
}
