function part1()
    horiz = 0
    depth = 0
    open("2.input") do f
        for line in readlines(f)
            line = split(line)
            dir = line[1]
            num = parse(Int64, line[2])
            if dir == "forward"
                horiz += num
            elseif dir == "down"
                depth += num
            elseif dir == "up"
                depth -= num
            else
                error("bad direction '$dir'")
            end
        end
    end
    println("horiz=$horiz depth=$depth ans=$(horiz*depth)")
end

function part2()
    horiz = 0
    depth = 0
    aim = 0
    open("2.input") do f
        for line in readlines(f)
            line = split(line)
            dir = line[1]
            num = parse(Int64, line[2])
            if dir == "forward"
                horiz += num
                depth += aim * num
            elseif dir == "down"
                aim += num
            elseif dir == "up"
                aim -= num
            else
                error("bad direction '$dir'")
            end
        end
    end
    println("horiz=$horiz depth=$depth aim=$aim ans=$(horiz*depth)")
end

part1()
part2()