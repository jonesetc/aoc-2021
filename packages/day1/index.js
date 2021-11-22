addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request));
});

async function handleRequest(request) {
  const response = await fetch(`https://adventofcode.com/2020/day/1/input`, {
    headers: {
        Cookie: `session=${AOC_SESSION}`
    }
  });
  const rawInput = await response.text();
  const lines = rawInput.split('\n').map(rawLine => parseInt(rawLine, 10));

  const seen = new Set();
  let answer = 0;
  lines.forEach(line => {
    const compliment = 2020 - line;

    if (seen.has(compliment)) {
      answer = line * compliment;
    } else {
      seen.add(line);
    }
  })

  return new Response(answer.toString());
}
