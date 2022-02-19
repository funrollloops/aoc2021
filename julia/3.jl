function part1()
    num_lines = 0
    cnts = Array{Int64}(undef, 0)
    N = 0
    open("3.input") do f
        for line in readlines(f)
            num_lines += 1
            line = strip(line)
            if N == 0
                N = length(line)
                println("N=$N")
                cnts = Array{Int64}(undef, N)
            end
            for i = 1:N
                if line[i] == '1'
                    cnts[i] += 1
                end
            end
        end
    end
    println("num_lines=$num_lines")
    𝛾 = 0
    𝜀 = 0
    for i = 1:N
        𝛾 <<= 1
        𝜀 <<= 1
        if cnts[i] > num_lines/2
            𝛾 |= 1
        else
            𝜀 |= 1
        end
    end
    println("𝛾=$𝛾 𝜀=$𝜀 ans=$(𝛾*𝜀)")
end

part1()