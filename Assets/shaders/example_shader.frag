#version 330 core
        out vec4 FragColor;
        in vec2 texCoord;
        in vec3 Color;
        uniform bool has_texture;
        uniform sampler2D texture1;
        void main()
        {
            if (has_texture)
            {
                FragColor = texture(texture1 ,texCoord) * vec4(Color,1.0); 
            }
            else
            {
                FragColor = vec4(Color,1.0); 
            }   
             
        }