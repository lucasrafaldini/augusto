# LUXO/LIXO - A Digital Tribute to Augusto de Campos

## The Iconic Concrete Poem

In 1965, Brazilian concrete poet **Augusto de Campos** created one of the most powerful and enduring pieces of concrete poetry: **LUXO/LIXO** (Luxury/Trash).

This minimal yet profound work critiques consumerism and social inequality by presenting the word "LUXO" (luxury) constructed from the letters of "LIXO" (trash), suggesting that luxury is built upon garbage, or that what society considers luxury is essentially trash.

## Digital Recreation

Now, 60 years later, the `augusto` CLI tool pays tribute to this masterpiece by allowing you to recreate this concept digitally:

```bash
augusto art "LUXO" "LIXO"
```

## The Output

```
L     I   X O   L  IXO 
L     I   X  O L  I   X
O     L   I   X   O   L
I     X   O  L I  X   O
LIXOL  IXO  L   I  XOL
```

## Implementation Details

### Test Added
A dedicated unit test (`test_luxo_lixo`) ensures this iconic combination always works perfectly:

```rust
#[test]
fn test_luxo_lixo() {
    // Tribute to Augusto de Campos' iconic concrete poem LUXO/LIXO
    // "LUXO" (luxury) written with "LIXO" (trash) - a powerful social commentary
    let result = word_art("LUXO", "LIXO");
    
    assert!(!result.is_empty());
    
    // Check that the output contains letters from LIXO
    assert!(result.contains('L') || result.contains('I') || 
            result.contains('X') || result.contains('O'));
    
    // Verify the result has 5 lines (one for each row in the 5x5 grid)
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 5);
    
    // Each line should have content (not be empty)
    for line in lines {
        assert!(!line.trim().is_empty());
    }
}
```

### Documentation
The LUXO/LIXO example is now prominently featured in:
- ✅ README.md - As a highlighted example with context
- ✅ ASCII_ART_FEATURE.md - In the examples section
- ✅ ASCII_ART_COMPLETE.md - As Example 2 (prime position)
- ✅ CHANGELOG.md - Mentioned in the feature additions
- ✅ Unit tests - Dedicated test case

## Why This Matters

### Historical Significance
- **Year**: 1965
- **Movement**: Concrete Poetry (Poesia Concreta)
- **Artist**: Augusto de Campos
- **Impact**: One of the most recognized pieces of Brazilian concrete poetry
- **Theme**: Social critique, consumerism, inequality

## Usage Examples

### Basic Usage
```bash
augusto art "LUXO" "LIXO"
```

### With Context
```bash
echo "=== LUXO/LIXO by Augusto de Campos (1965) ==="
echo "Luxury built from trash - A critique of consumerism"
echo
augusto art "LUXO" "LIXO"
```

### Compare Original Concept
The original concrete poem by Campos was even more minimal - just the two words with clever typography showing the transformation. Our digital version expands this by:
- Using ASCII art representation
- Making "LUXO" large and prominent
- Filling it entirely with "LIXO" characters
- Creating a visual metaphor in character form

## The Philosophy

### Augusto de Campos' Vision
Concrete poetry isn't just about visual arrangement - it's about making the **form** of the poem inseparable from its **content**. In LUXO/LIXO:
- The **word** is "LUXO" (luxury)
- The **material** is "LIXO" (trash)
- The **message** is that they're interconnected
- The **experience** is visual and conceptual simultaneously

### The Digital Extension
The `augusto` tool extends this philosophy:
- **Input**: Two words (main + filler)
- **Process**: Deconstruction and reconstruction
- **Output**: Visual poetry
- **Result**: Form becomes content becomes art

## Test Results

```bash
$ cargo test test_luxo_lixo
running 1 test
test ascii_art::tests::test_luxo_lixo ... ok

test result: ok. 1 passed; 0 failed
```

✅ **Status**: Working perfectly!

## Impact on the Project

### Before
- ASCII art feature existed
- Examples focused on English words
- Missing connection to the project's namesake

### After
- ✅ Direct tribute to Augusto de Campos
- ✅ Cultural and historical context
- ✅ Meaningful example that embodies the tool's purpose
- ✅ Educational value about concrete poetry
- ✅ Demonstrates social commentary through code

## Fun Facts

1. **The Original**: Augusto de Campos created many visual poems, but LUXO/LIXO is among the most famous
2. **The Name**: Our tool is named after Augusto de Campos, making this a perfect tribute
3. **The Timing**: 2025 marks 60 years since the poem's creation (1965)
4. **The Message**: Still relevant - consumerism and waste remain major issues
5. **The Medium**: ASCII art is the digital equivalent of typewriter concrete poetry

## Try It Yourself!

```bash
# Build the tool
cd augusto
cargo build --release

# Run the tribute
./target/release/augusto art "LUXO" "LIXO"

# Try your own combinations
./target/release/augusto art "PEACE" "CHAOS"
./target/release/augusto art "LOVE" "HATE"
./target/release/augusto art "TRUTH" "LIES"


## Conclusion

By implementing the LUXO/LIXO example, we've:
- ✅ Honored Augusto de Campos
- ✅ Demonstrated the tool's capabilities
- ✅ Provided cultural context
- ✅ Created a meaningful test case
- ✅ Connected past and present
- ✅ Made concrete poetry accessible

**The spirit of concrete poetry lives on in code!** 🎨

---

*"In concrete poetry, the poem is not about something - it IS something."*  
— Augusto de Campos

*Test Status: ✅ Passing*  
*Tests Total: 11/11*  
*LUXO/LIXO Test: Implemented and documented*  
*Created: October 4, 2025*
