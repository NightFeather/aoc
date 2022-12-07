require 'set'

it = ARGF.each_char.each_with_index

puts it.reduce([]) { |s, (v, i)|
  s << v
  s.shift if s.length > 4
  if s.uniq.length == 4
    break i
  end
  s
} + 1

it.rewind

puts it.reduce([]) { |s, (v, i)|
  s << v
  s.shift if s.length > 14
  if s.uniq.length == 14
    break i
  end
  s
} + 1
