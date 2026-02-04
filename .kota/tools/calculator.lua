-- Example: Calculator tool
kota.register_tool({
    name = "calculator",
    description = "Perform basic arithmetic operations (add, subtract, multiply, divide)",
    parameters = {
        type = "object",
        properties = {
            operation = {
                type = "string",
                description = "The operation to perform: add, subtract, multiply, divide",
                enum = { "add", "subtract", "multiply", "divide" }
            },
            a = {
                type = "number",
                description = "First number"
            },
            b = {
                type = "number",
                description = "Second number"
            }
        },
        required = { "operation", "a", "b" }
    },
    handler = function(args)
        local op = args.operation
        local a = args.a
        local b = args.b
        
        if op == "add" then
            return { result = a + b, operation = op }
        elseif op == "subtract" then
            return { result = a - b, operation = op }
        elseif op == "multiply" then
            return { result = a * b, operation = op }
        elseif op == "divide" then
            if b == 0 then
                return { error = "Division by zero" }
            end
            return { result = a / b, operation = op }
        else
            return { error = "Unknown operation: " .. op }
        end
    end
})
