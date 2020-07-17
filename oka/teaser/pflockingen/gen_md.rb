#!/usr/bin/env ruby
# frozen_string_literal: true

# puts '# All images from Pflockingen Institute'
(16..42).each do |i|
  d = format('%02d', i)
  puts "## message #{d}

![message #{d}](./annotated#{d}.svg)

"
end
