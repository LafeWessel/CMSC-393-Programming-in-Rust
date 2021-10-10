/*

		a = new UnlimitedInteger("12"); System.out.println("A is " + a);
		b = new UnlimitedInteger("7"); System.out.println("B is " + b);

		c = (new UnlimitedInteger("5")).factorial();
		System.out.print("According to you, 5! = " + c);
		if (c.toString().equals("120")) System.out.println("  Correct"); else System.out.println("  Wrong!");

		a = new UnlimitedInteger("487652"); System.out.println("A is " + a);
		b = new UnlimitedInteger("365396"); System.out.println("B is " + b);

		c = (new UnlimitedInteger("22")).factorial();
		System.out.print("According to you, 22! = " + c);
		if (c.toString().equals("1124000727777607680000")) System.out.println("  Correct"); else System.out.println("  Wrong!");

		// Factorial
		System.out.println("------------------ Factorial ------------------");
		for (int i = 0; i < num_of_probs_to_check; i++) {
			int temp = rnd.nextInt(100)+50;
			BigInteger x = new BigInteger(Integer.toString(temp,10));

			a = new UnlimitedInteger(Integer.toString(temp, 10));
			c = a.factorial();
			System.out.println(a + "! = " + c);
			if (c.toString().equals(bigIntFactorial(x).toString())) {
				System.out.println("That is correct.\n");
			} else {
				System.out.println("That is wrong.\n");
			}
		}

 */