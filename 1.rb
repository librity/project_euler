# frozen_string_literal: true

def sum_of_multiples_of_3_and_5(up_to)
  (1...up_to).select { |n| (n % 3).zero? || (n % 5).zero? }.sum
end

puts sum_of_multiples_of_3_and_5 10
puts sum_of_multiples_of_3_and_5 1000
