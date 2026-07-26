print("========================================")
print("  Lua 5.4.6 running on Maarch64 Engine  ")
print("========================================")

local sum = 0
for i = 1, 10 do
    sum = sum + i
end
print("Sum 1..10 = " .. sum)

local function fib(n)
    if n <= 1 then return n end
    return fib(n - 1) + fib(n - 2)
end

print("Fibonacci(15) = " .. fib(15))
