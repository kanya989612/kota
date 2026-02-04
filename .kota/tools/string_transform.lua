-- Example: String manipulation tool
kota.register_tool({
    name = "string_transform",
    description = "Transform strings (uppercase, lowercase, reverse, length)",
    parameters = {
        type = "object",
        properties = {
            text = {
                type = "string",
                description = "The text to transform"
            },
            operation = {
                type = "string",
                description = "The transformation: uppercase, lowercase, reverse, length",
                enum = { "uppercase", "lowercase", "reverse", "length" }
            }
        },
        required = { "text", "operation" }
    },
    handler = function(args)
        local text = args.text
        local op = args.operation
        
        if op == "uppercase" then
            return { result = string.upper(text) }
        elseif op == "lowercase" then
            return { result = string.lower(text) }
        elseif op == "reverse" then
            return { result = string.reverse(text) }
        elseif op == "length" then
            return { result = string.len(text) }
        else
            return { error = "Unknown operation: " .. op }
        end
    end
})
