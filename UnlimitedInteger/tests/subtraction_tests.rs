/*
		a = new UnlimitedInteger("12"); System.out.println("A is " + a);
		b = new UnlimitedInteger("7"); System.out.println("B is " + b);


		c = (a.subtract(b));
		System.out.print("According to you, " + a + " - " + b + " = " + c);
		if (c.toString().equals("5")) System.out.println("  Correct"); else System.out.println("  Wrong!");

		a = new UnlimitedInteger("487652"); System.out.println("A is " + a);
		b = new UnlimitedInteger("365396"); System.out.println("B is " + b);

		c = (a.subtract(b));
		System.out.print("According to you, " + a + " - " + b + " = " + c);
		if (c.toString().equals("122256")) System.out.println("  Correct"); else System.out.println("  Wrong!");


				// Subtraction
		System.out.println("------------------ Subtraction ------------------");
		for (int i = 0; i < num_of_probs_to_check; i++) {
			BigInteger x = new BigInteger(num_of_bits_per_number, rnd);
			BigInteger y = new BigInteger(num_of_bits_per_number, rnd);

			a = new UnlimitedInteger(x.toString());
			b = new UnlimitedInteger(y.toString());
			if (x.compareTo(y) > 0) {
				c = a.subtract(b);
				System.out.println(a + "\n - \n" + b + "\n = \n" + c);
				if (c.toString().equals(x.subtract(y).toString())) { //needs if statement to determine correct answer, as we added if statement above
					System.out.println("That is correct.\n");
				} else {
					System.out.println("That is wrong.\n");
				}
			} else {
				c = b.subtract(a);
				System.out.println(b + "\n - \n" + a + "\n = \n" + c);
				if (c.toString().equals(y.subtract(x).toString())) { //I edited this part by adding if statement to make sure it subtracts in the correct direction
					System.out.println("That is correct.\n");
				} else {
					System.out.println("That is wrong.\n");
				}
			}
		}

 */