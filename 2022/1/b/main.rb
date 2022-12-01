puts (ARGF.each_line.reduce([0]) do |o, l|
  if l.chomp.empty?
    o << 0
  else
    o[-1] += l.chomp.to_i
  end
  o
end).sort[-1]

