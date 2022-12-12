require 'shellwords'

r = ARGF.each_line.reduce([[],{}]) { |(s, fs), l|
  if l.start_with? '$'
    exe, *args = Shellwords.shellsplit(l)[1..-1]
    if exe == 'ls'
    elsif exe == 'cd'
      if args.first == '..'
        s.pop
      else
        s.push args.first
        fs[s.join("/")] ||= 0
      end
    end
  elsif l.start_with? 'dir'
    _, name = l.chomp.split
    fs[(s+[name]).join("/")] = 0
  else
    sz, name = l.chomp.split
    sz = sz.to_i
    _s = s.dup
    loop do
      break if _s.empty?
      fs[_s.join("/")] += sz
      _s.pop
    end
  end
  [s,fs]
}

s, fs = r

puts fs.select { |k,v| v < 100000 }.values.sum
puts fs.select { |k,v| v >= 30000000 - (70000000 - fs["/"]) }.sort_by(&:last).first.last
