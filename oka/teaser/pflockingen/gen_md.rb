#!/usr/bin/env ruby
# frozen_string_literal: true

puts '# All images from Pflockingen Institute'
(1..15).each do |i|
  d = format('%02d', i)
  puts "## image#{d}

![image#{d}](./annotated#{d}.svg)

"
end
