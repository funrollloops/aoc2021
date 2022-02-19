
function part1()
    num_lines = 0
    increases = 0
    last = 999999999999
    open("1.input") do f
        for line in readlines(f)
            num = parse(Int64, line)
            num_lines += 1
            if num > last
                increases += 1
            end
            last = num
        end
    end

    println("line count is $num_lines")
    println("increases = $increases")
end

function part2()
    increases = 0
    n0 = 999999999999
    n1 = 999999999999
    n2 = 999999999999
    open("1.input") do f
        for line in readlines(f)
            num = parse(Int64, line)
            if num > n0
                increases += 1
            end
            n0 = n1
            n1 = n2
            n2 = num
        end
    end
    println("increases = $increases")
end

part1()

part2()