const fs = require('fs')

const input = fs.readFileSync("input.txt", 'utf-8')

function parseOperands(expr) {
    const isNumber = c => !isNaN(parseFloat(c));
    return expr.split(',')
        .map(substr => substr.split('').filter(isNumber).join(''))
        .map(parseFloat)
}

function part1() {
    const re = /mul\([0-9]+,[0-9]+\)/g
    const exprs = input.match(re) || []
    let sum = 0;
    for (const expr of exprs) {
        const operands = parseOperands(expr)
        sum += operands[0] * operands[1]
    }

    console.log("Part 1: ", sum)
}

function part2() {
    const re = /mul\([0-9]+,[0-9]+\)|don't\(\)|do\(\)/g
    const exprs = input.match(re) || []
    let sum = 0
    let enabled = true
    for (const expr of exprs) {
        if (expr == "don't()") {
            enabled = false
            continue
        }
        if (expr == "do()") {
            enabled = true
            continue
        }
        if (enabled) {
            const operands = parseOperands(expr)
            sum += operands[0] * operands[1]
        }
    }

    console.log("Part 2: ", sum)
}

part1()
part2()