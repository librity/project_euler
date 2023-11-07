# frozen_string_literal: true

def fibonnaci(nth)
  return 1 if nth < 2

  fibonnaci(nth - 1) + fibonnaci(nth - 2)
end

def fibonnaci_even_sum(up_to)
  (1..up_to).map { |n| fibonnaci n }.select { |n| (n % 2).zero? }.sum
end

puts fibonnaci 10
puts fibonnaci_even_sum 4_000_000
